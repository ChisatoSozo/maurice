use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct Song {
    pub thumbnail: String,
    pub title: String,
    pub url: String,
}

struct Speaker {
    device_name: String,
    now_playing: Option<Song>,
    queue: Vec<Song>,
    socket_path: String,
    connection: Arc<Mutex<Option<UnixStream>>>,
}

pub struct MultiSpeaker {
    speakers: HashMap<String, Speaker>,
}

impl Speaker {
    fn new(device_name: String) -> Self {
        Self {
            device_name,
            now_playing: None,
            queue: Vec::new(),
            socket_path: String::from("/tmp/mpvsocket"),
            connection: Arc::new(Mutex::new(None)),
        }
    }

    fn play(&mut self, song: Song) -> Result<()> {
        //if there is a song playing, and it's the same song, resume playback, or do nothing
        if let Some(now_playing) = &self.now_playing {
            if now_playing.url == song.url {
                if self.get_paused()? {
                    return self.resume();
                } else {
                    return Ok(());
                }
            }
        }

        self.stop_current_playback();
        let dev_name = &self.device_name;
        Command::new("mpv")
            .args(&[
                format!("--audio-device=pulse/{dev_name}").as_str(),
                "--no-video",
                &song.url,
                &format!("--input-ipc-server={}", self.socket_path),
            ])
            .spawn()?;
        self.now_playing = Some(song);
        Ok(())
    }

    fn pause(&mut self) -> Result<()> {
        self.send_command("set_property", "pause", "yes")
    }

    fn resume(&mut self) -> Result<()> {
        self.send_command("set_property", "pause", "no")
    }

    fn set_volume(&mut self, volume: f64) -> Result<()> {
        self.send_command("set_property", "volume", &volume.to_string())
    }

    fn get_volume(&self) -> Result<f64> {
        match self.get_property("volume")? {
            Value::Number(volume) => Ok(volume.as_f64().ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid volume value: {}", volume),
            ))? as f64),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid volume value")),
        }
    }

    fn stop_current_playback(&mut self) {
        let _ = Command::new("pkill").arg("-f").arg("mpv").status();
        self.now_playing = None;
        // Reset the connection when stopping playback
        let mut conn = self.connection.lock().unwrap();
        *conn = None;
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
            // Read and discard the response
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

    fn get_current_time(&self) -> Result<String> {
        match self.get_property("time-pos")? {
            Value::Number(time) => Ok(time.to_string()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid time-pos value")),
        }
    }

    fn get_duration(&self) -> Result<String> {
        match self.get_property("duration")? {
            Value::Number(duration) => Ok(duration.to_string()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid duration value")),
        }
    }

    fn enqueue(&mut self, song: Song) {
        self.queue.push(song);
    }

    fn remove_song(&mut self, index: usize) -> Result<()> {
        if index >= self.queue.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid index"));
        }
        self.queue.remove(index);
        Ok(())
    }

    fn emplace_song(&mut self, song: Song, index: usize) -> Result<()> {
        if index <= self.queue.len() {
            self.queue.insert(index, song);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid index"))
        }
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

    pub fn add_speaker(&mut self, name: String, device_name: String) {
        self.speakers.insert(name, Speaker::new(device_name));
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

    pub fn get_current_time(&self, speaker_name: &str) -> Result<String> {
        self.get_speaker(speaker_name)?.get_current_time()
    }

    pub fn get_duration(&self, speaker_name: &str) -> Result<String> {
        self.get_speaker(speaker_name)?.get_duration()
    }

    pub fn enqueue(&mut self, speaker_name: &str, song: Song) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.enqueue(song);
        Ok(())
    }

    pub fn remove_song(&mut self, speaker_name: &str, index: usize) -> Result<()> {
        self.get_speaker_mut(speaker_name)?.remove_song(index)
    }

    pub fn emplace_song(&mut self, speaker_name: &str, song: Song, index: usize) -> Result<()> {
        self.get_speaker_mut(speaker_name)?
            .emplace_song(song, index)
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

#[test]
fn test_multi_speaker_functionality() {
    use crate::procedures::get_audio_devices::get_audio_devices;
    let mut multi_speaker = MultiSpeaker::new();
    let devices = get_audio_devices().unwrap();
    multi_speaker.add_speaker(
        "test_speaker".to_string(),
        devices.get(0).unwrap().name.clone(),
    );

    let test_song = Song {
        thumbnail: "https://example.com/thumbnail.jpg".to_string(),
        title: "Test Song".to_string(),
        url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
    };

    // Play the song
    assert!(multi_speaker
        .play("test_speaker", test_song.clone())
        .is_ok());
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Test pause
    assert!(multi_speaker.pause("test_speaker").is_ok());
    std::thread::sleep(std::time::Duration::from_secs(2));
    assert!(multi_speaker.get_paused("test_speaker").unwrap());

    // Test resume
    assert!(multi_speaker.resume("test_speaker").is_ok());
    std::thread::sleep(std::time::Duration::from_secs(2));
    assert!(!multi_speaker.get_paused("test_speaker").unwrap());

    // Get current time and duration
    let current_time = multi_speaker.get_current_time("test_speaker").unwrap();
    let duration = multi_speaker.get_duration("test_speaker").unwrap();
    println!("Current time: {}, Duration: {}", current_time, duration);

    // Stop playback
    multi_speaker
        .get_speaker_mut("test_speaker")
        .unwrap()
        .stop_current_playback();

    // wait for the process to stop
    std::thread::sleep(std::time::Duration::from_secs(2));

    //start playback again
    assert!(multi_speaker
        .play("test_speaker", test_song.clone())
        .is_ok());

    std::thread::sleep(std::time::Duration::from_secs(5));
}
