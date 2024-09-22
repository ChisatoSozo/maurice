use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::os::unix::net::UnixStream;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use libpulse_binding::volume::{ChannelVolumes, Volume};
use paperclip::actix::Apiv2Schema;
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema, PartialEq)]
pub struct Song {
    pub thumbnail: String,
    pub title: String,
    pub url: String,
}

struct Speaker {
    device_name: String,
    queue: Arc<Mutex<Vec<Song>>>,
    socket_path: String,
    connection: Arc<Mutex<Option<UnixStream>>>,
    pulse_controller: Arc<Mutex<SinkController>>,
    volume: f64,
    current_process: Arc<Mutex<Option<Child>>>,
    current_song: Arc<Mutex<Option<Song>>>,
}

pub struct MultiSpeaker {
    speakers: HashMap<String, Speaker>,
}

unsafe impl Send for Speaker {}

fn get_song_time(stream: &mut UnixStream) -> Result<f64> {
    let command = json!({
        "command": ["get_property", "time-pos"]
    });
    writeln!(stream, "{}", command.to_string())?;
    stream.flush()?;
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    let parsed: serde_json::Value = serde_json::from_str(&response)?;
    match parsed["data"].clone() {
        Value::Number(time) => Ok(time.as_f64().ok_or(Error::new(
            ErrorKind::InvalidData,
            "Invalid time-pos value, not f64",
        ))?),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid time-pos value, not number",
        )),
    }
}

impl Speaker {
    fn new(device_name: String) -> Result<Self> {
        let mut this = Self {
            device_name,
            queue: Arc::new(Mutex::new(Vec::new())),
            socket_path: String::from("/tmp/mpvsocket"),
            connection: Arc::new(Mutex::new(None)),
            pulse_controller: Arc::new(Mutex::new(SinkController::create().unwrap())),
            volume: 100.0,
            current_process: Arc::new(Mutex::new(None)),
            current_song: Arc::new(Mutex::new(None)),
        };
        this.set_volume(100.0)?;
        Ok(this)
    }

    fn play(&mut self, song: Song) -> Result<()> {
        let current_song = self.current_song.lock().unwrap().clone();

        if current_song.as_ref() == Some(&song) {
            // If the requested song is already playing, do nothing
            return Ok(());
        }

        // Stop the current process if it exists
        if let Some(mut child) = self.current_process.lock().unwrap().take() {
            let _ = child.kill();
        }

        let dev_name = self.device_name.clone();
        let socket_path = self.socket_path.clone();

        let child = Command::new("mpv")
            .args(&[
                format!("--audio-device=pulse/{dev_name}").as_str(),
                "--no-video",
                &song.url,
                &format!("--input-ipc-server={}", socket_path),
            ])
            .spawn()?;

        *self.current_process.lock().unwrap() = Some(child);
        *self.current_song.lock().unwrap() = Some(song.clone());

        // Clear the queue and add the new song
        {
            let mut queue = self
                .queue
                .lock()
                .map_err(|e| Error::new(ErrorKind::Other, format!("Error locking queue: {}", e)))?;
            queue.clear();
            queue.push(song);
        }

        // Clone necessary data for the thread

        let queue = self.queue.clone();
        let device_name = self.device_name.clone();
        let socket_path = self.socket_path.clone();
        let current_song = self.current_song.clone();
        let connection = self.connection.clone();

        // Start a thread to monitor playback and advance the queue
        thread::spawn(move || {
            loop {
                // Attempt to get the current time of the song
                let mut conn = connection.lock().unwrap();
                if let Some(stream) = conn.as_mut() {
                    println!("Getting song time");
                    match get_song_time(stream) {
                        Ok(_) => {}
                        Err(e) => {
                            let error_str = format!("Error getting song time: {}", e);
                            println!("{}", error_str);
                            if error_str.contains("Broken pipe") {
                                break;
                            }
                        }
                    };
                }
                drop(conn);
                thread::sleep(Duration::from_secs(1));
            }
            // If there's a next song, play it, otherwise, break
            println!("Playing next song");

            // Play the next song
            let mut queue_lock = queue.lock().unwrap();
            if queue_lock.len() > 1 {
                queue_lock.remove(0); // Remove the current song
                if let Some(next_song) = queue_lock.get(0).cloned() {
                    drop(queue_lock); // Release the lock before spawning a new process

                    // Spawn a new process for the next song
                    let child = Command::new("mpv")
                        .args(&[
                            format!("--audio-device=pulse/{}", device_name).as_str(),
                            "--no-video",
                            &next_song.url,
                            &format!("--input-ipc-server={}", socket_path),
                        ])
                        .spawn();

                    match child {
                        Ok(_) => {
                            // Update the current song
                            *current_song.lock().unwrap() = Some(next_song);
                        }
                        Err(e) => {
                            eprintln!("Error playing next song: {}", e);
                        }
                    }
                }
            } else {
                //clear queue
                queue_lock.clear();
            }
        });

        Ok(())
    }

    // Helper function to get song time

    fn pause(&mut self) -> Result<()> {
        self.send_command("set_property", "pause", "yes")
    }

    fn resume(&mut self) -> Result<()> {
        self.send_command("set_property", "pause", "no")
    }

    fn set_volume(&mut self, volume: f64) -> Result<()> {
        let mut controller = self.pulse_controller.lock().unwrap();
        let devices = controller
            .list_devices()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        let device = devices
            .iter()
            .find(|d| d.name == Some(self.device_name.clone()));

        if let Some(dev) = device {
            let pulse_volume = Volume(((0x10000 as f64) * volume / 100.0).floor() as u32);
            controller.set_device_volume_by_index(
                dev.index,
                &ChannelVolumes::default().set(2, pulse_volume),
            );
            self.volume = volume;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Device not found"))
        }
    }

    fn get_volume(&self) -> Result<f64> {
        Ok(self.volume)
    }

    fn ensure_connection(&self) -> Result<()> {
        let mut conn = self.connection.lock().unwrap();
        if conn.is_none() {
            *conn = Some(UnixStream::connect(&self.socket_path)?);
            conn.as_mut()
                .unwrap()
                .set_read_timeout(Some(Duration::from_secs(1)))?;
            conn.as_mut()
                .unwrap()
                .set_write_timeout(Some(Duration::from_secs(1)))?;
        }
        Ok(())
    }

    fn send_command(&self, command: &str, property: &str, value: &str) -> Result<()> {
        self.ensure_connection()?;
        let command = json!({
            "command": [command, property, value]
        });
        let mut conn = self.connection.lock().unwrap();
        if let Some(stream) = conn.as_mut() {
            writeln!(stream, "{}", command.to_string())?;
            stream.flush()?;
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response)?;
        }
        Ok(())
    }

    fn get_property(&self, property: &str) -> Result<Value> {
        self.ensure_connection()?;
        let command = json!({
            "command": ["get_property", property]
        });
        let mut conn = self.connection.lock().unwrap();
        if let Some(stream) = conn.as_mut() {
            writeln!(stream, "{}", command.to_string())?;
            stream.flush()?;
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response)?;
            let parsed: serde_json::Value = serde_json::from_str(&response)?;
            Ok(parsed["data"].clone())
        } else {
            Err(Error::new(ErrorKind::Other, "No active connection"))
        }
    }

    fn get_paused(&self) -> Result<bool> {
        match self.get_property("pause")? {
            Value::Bool(paused) => Ok(paused),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid pause value")),
        }
    }

    fn get_song_time(&self) -> Result<f64> {
        match self.get_property("time-pos")? {
            Value::Number(time) => Ok(time.as_f64().ok_or(Error::new(
                ErrorKind::InvalidData,
                "Invalid time-pos value, not f64",
            ))?),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid time-pos valu, not number",
            )),
        }
    }

    fn get_duration(&self) -> Result<f64> {
        match self.get_property("duration")? {
            Value::Number(duration) => Ok(duration.as_f64().ok_or(Error::new(
                ErrorKind::InvalidData,
                "Invalid duration value, not f64",
            ))?),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid duration value")),
        }
    }

    fn set_song_time(&self, time: f64) -> Result<()> {
        self.send_command("set_property", "time-pos", &time.to_string())
    }

    fn set_loop(&self, looping: bool) -> Result<()> {
        self.send_command(
            "set_property",
            "loop-file",
            if looping { "inf" } else { "no" },
        )
    }

    fn get_playlist(&mut self) -> Result<Vec<Song>> {
        Ok(self
            .queue
            .lock()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Error locking queue: {}", e)))?
            .clone())
    }

    fn append_song_to_playlist(&mut self, song: Song) -> Result<()> {
        let queue = self
            .queue
            .lock()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Error locking queue: {}", e)))?;

        if queue.is_empty() {
            // Release the lock before calling self.play()
            drop(queue);
            self.play(song)?;
        } else {
            drop(queue);
            let mut queue = self
                .queue
                .lock()
                .map_err(|e| Error::new(ErrorKind::Other, format!("Error locking queue: {}", e)))?;
            queue.push(song);
        }
        Ok(())
    }

    fn remove_song_from_playlist_at_index(&mut self, index: usize) -> Result<()> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Error locking queue: {}", e)))?;
        if index >= queue.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
        }
        queue.remove(index);
        Ok(())
    }
}
impl MultiSpeaker {
    pub fn new() -> Self {
        Self {
            speakers: HashMap::new(),
        }
    }

    pub fn has_speaker(&self, name: &str) -> bool {
        self.speakers.contains_key(name)
    }

    pub fn list_speakers(&self) -> Vec<String> {
        self.speakers.keys().cloned().collect()
    }

    pub fn add_speaker(&mut self, name: String, device_name: String) -> Result<()> {
        self.speakers.insert(name, Speaker::new(device_name)?);
        Ok(())
    }

    pub fn play(&mut self, speaker_name: &str, song: Song) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.play(song)
    }

    pub fn pause(&mut self, speaker_name: &str) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.pause()
    }

    pub fn resume(&mut self, speaker_name: &str) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.resume()
    }

    pub fn set_volume(&mut self, speaker_name: &str, volume: f64) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.set_volume(volume)
    }

    pub fn get_volume(&self, speaker_name: &str) -> Result<f64> {
        self.get_speaker(speaker_name)?.get_volume()
    }

    pub fn get_paused(&self, speaker_name: &str) -> Result<bool> {
        self.get_speaker(speaker_name)?.get_paused()
    }

    pub fn get_song_time(&self, speaker_name: &str) -> Result<f64> {
        self.get_speaker(speaker_name)?.get_song_time()
    }

    pub fn get_duration(&self, speaker_name: &str) -> Result<f64> {
        self.get_speaker(speaker_name)?.get_duration()
    }

    pub fn set_song_time(&mut self, speaker_name: &str, time: f64) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.set_song_time(time)
    }

    pub fn set_loop(&mut self, speaker_name: &str, looping: bool) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.set_loop(looping)
    }

    pub fn get_playlist(&mut self, speaker_name: &str) -> Result<Vec<Song>> {
        Ok(self.get_speaker_mut(speaker_name)?.get_playlist()?)
    }

    pub fn append_song_to_playlist(&mut self, speaker_name: &str, song: Song) -> Result<()> {
        self.get_speaker_mut(speaker_name)?
            .append_song_to_playlist(song)
    }

    pub fn remove_song_from_playlist_at_index(
        &mut self,
        speaker_name: &str,
        index: usize,
    ) -> Result<()> {
        self.get_speaker_mut(speaker_name)?
            .remove_song_from_playlist_at_index(index)
    }

    fn get_speaker(&self, name: &str) -> Result<&Speaker> {
        self.speakers
            .get(name)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Speaker not found"))
    }

    fn get_speaker_mut(&mut self, name: &str) -> Result<&mut Speaker> {
        self.speakers
            .get_mut(name)
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Speaker not found"))
    }
}
