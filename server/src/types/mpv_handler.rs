use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    process::Child,
    sync::mpsc,
};

use libpulse_binding::volume::{ChannelVolumes, Volume};
use log::error;
use paperclip::actix::Apiv2Schema;
use pulsectl::controllers::{DeviceControl, SinkController};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema, PartialEq)]
pub struct Song {
    pub thumbnail: String,
    pub title: String,
    pub url: String,
}

pub enum MpvHandlerMessage {
    Play(String),
    Pause(String),
    Stop(String),
    Next(String),
    Seek(String, f64),
    Volume(String, f64),
    GetVolume(String),
    Add(String, Song),
    Remove(String, usize),
    Clear(String),
    List(String),
    TimeAndDuration(String),
    AddDevice(String),
    RemoveDevice(String),
}

pub enum MpvHandlerResponse {
    Ok,
    Error(String),
    List(Vec<Song>),
    Volume(f64),
    TimeAndDuration(f64, f64),
}

pub struct MpvHandlerState {
    queue: Vec<Song>,
    mpv_process: Option<Child>,
    mpv_sock: Option<UnixStream>,
    volume: f64,
}

impl MpvHandlerState {
    fn new() -> Self {
        Self {
            queue: Vec::new(),
            mpv_process: None,
            mpv_sock: None,
            volume: 100.0,
        }
    }
}

pub struct MpvHandler {
    state: HashMap<String, MpvHandlerState>,
    consumer: mpsc::Receiver<MpvHandlerMessage>,
    producer: mpsc::Sender<MpvHandlerResponse>,
    pulse_controller: SinkController,
}

fn send_command(
    stream: &mut UnixStream,
    command: &str,
    property: &str,
    value: &str,
) -> Result<(), String> {
    let command = json!({"command": [command, property, value]});
    writeln!(stream, "{}", command.to_string()).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;
    BufReader::new(stream).lines().next(); // Read and discard response
    Ok(())
}

fn send_command_single_arg(
    stream: &mut UnixStream,
    command: &str,
    arg: &str,
) -> Result<(), String> {
    let command = json!({"command": [command, arg]});
    writeln!(stream, "{}", command.to_string()).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;
    BufReader::new(stream).lines().next(); // Read and discard response
    Ok(())
}

fn get_property(stream: &mut UnixStream, property: &str) -> Result<Value, String> {
    let command = json!({"command": ["get_property", property]});
    writeln!(stream, "{}", command.to_string()).map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;
    let response = BufReader::new(stream)
        .lines()
        .next()
        .ok_or("no line")
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    let parsed: Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
    Ok(parsed["data"].clone())
}

impl MpvHandler {
    pub fn new(
        consumer: mpsc::Receiver<MpvHandlerMessage>,
        producer: mpsc::Sender<MpvHandlerResponse>,
    ) -> Result<Self, String> {
        Ok(Self {
            state: HashMap::new(),
            consumer,
            producer,
            pulse_controller: SinkController::create().map_err(|e| e.to_string())?,
        })
    }

    fn run(&mut self) {
        loop {
            let message = self.consumer.recv().map_err(|e| e.to_string());
            let message = match message {
                Ok(message) => message,
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    continue;
                }
            };
            let response = self.handle_message(message);
            let send_result = self.producer.send(response).map_err(|e| e.to_string());
            match send_result {
                Ok(_) => {}
                Err(e) => {
                    error!("Error sending response: {}", e);
                }
            }
        }
    }

    fn handle_message(&mut self, message: MpvHandlerMessage) -> MpvHandlerResponse {
        match message {
            MpvHandlerMessage::AddDevice(id) => {
                if self.state.get(&id).is_none() {
                    self.state.insert(id.clone(), MpvHandlerState::new());
                } else {
                    return MpvHandlerResponse::Error("Device already exists".to_string());
                }
                MpvHandlerResponse::Ok
            }
            MpvHandlerMessage::RemoveDevice(id) => {
                if self.state.get(&id).is_some() {
                    self.state.remove(&id);
                } else {
                    return MpvHandlerResponse::Error("Device does not exist".to_string());
                }
                MpvHandlerResponse::Ok
            }
            MpvHandlerMessage::Play(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    if let Some(mpv_sock) = state.mpv_sock.borrow_mut() {
                        match send_command(mpv_sock, "set_property", "pause", "no") {
                            Ok(_) => {}
                            Err(e) => return MpvHandlerResponse::Error(e),
                        }

                        return MpvHandlerResponse::Ok;
                    }

                    //if the queue is not empty, play the first song
                    if !state.queue.is_empty() {
                        let song = state
                            .queue
                            .get(0)
                            .ok_or("no song")
                            .map_err(|e| MpvHandlerResponse::Error(e.to_string()));
                        let song = match song {
                            Ok(song) => song,
                            Err(e) => return e,
                        };
                        let mpv_process = std::process::Command::new("mpv")
                            .arg("--input-ipc-server")
                            .arg("--no-video")
                            .arg(format!("/tmp/mpv-socket-{}", id))
                            .arg(&song.url)
                            .spawn()
                            .map_err(|e| e.to_string())
                            .map_err(|e| MpvHandlerResponse::Error(e));
                        let mpv_process = match mpv_process {
                            Ok(mpv_process) => mpv_process,
                            Err(e) => return e,
                        };
                        let mpv_sock = UnixStream::connect(format!("/tmp/mpv-socket-{}", id))
                            .map_err(|e| e.to_string())
                            .map_err(|e| MpvHandlerResponse::Error(e));
                        let mut mpv_sock = match mpv_sock {
                            Ok(mpv_sock) => mpv_sock,
                            Err(e) => return e,
                        };

                        let command_result =
                            send_command(&mut mpv_sock, "set_property", "pause", "no")
                                .map_err(|e| e.to_string())
                                .map_err(|e| MpvHandlerResponse::Error(e));
                        state.mpv_process = Some(mpv_process);
                        state.mpv_sock = Some(mpv_sock);
                        match command_result {
                            Ok(_) => {}
                            Err(e) => return e,
                        }
                        return MpvHandlerResponse::Ok;
                    }

                    return MpvHandlerResponse::Error("Queue is empty".to_string());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Pause(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    if let Some(mpv_sock) = state.mpv_sock.borrow_mut() {
                        match send_command(mpv_sock, "set_property", "pause", "yes") {
                            Ok(_) => {}
                            Err(e) => return MpvHandlerResponse::Error(e),
                        }
                        return MpvHandlerResponse::Ok;
                    }

                    return MpvHandlerResponse::Error("No song is playing".to_string());
                }

                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Stop(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    if let Some(mpv_process) = &mut state.mpv_process {
                        let kill_result =
                            mpv_process.kill().map_err(|e| e.to_string()).map_err(|e| {
                                return MpvHandlerResponse::Error(e);
                            });
                        match kill_result {
                            Ok(_) => {}
                            Err(e) => return e,
                        }
                        state.mpv_process = None;
                        state.mpv_sock = None;
                        //if there's a song in the queue, remove the first song
                        if !state.queue.is_empty() {
                            state.queue.remove(0);
                        }
                        return MpvHandlerResponse::Ok;
                    }
                    return MpvHandlerResponse::Error("No song is playing".to_string());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Next(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    if let Some(_) = &state.mpv_sock {
                        //stop then play the next song
                        let response = self.handle_message(MpvHandlerMessage::Stop(id.clone()));
                        if let MpvHandlerResponse::Ok = response {
                            let response = self.handle_message(MpvHandlerMessage::Play(id));
                            return response;
                        }
                        return response;
                    }
                    return MpvHandlerResponse::Error("No song is playing".to_string());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Seek(id, time) => {
                if let Some(state) = self.state.get_mut(&id) {
                    if let Some(mpv_sock) = state.mpv_sock.borrow_mut() {
                        match send_command_single_arg(mpv_sock, "seek", time.to_string().as_str()) {
                            Ok(_) => {}
                            Err(e) => return MpvHandlerResponse::Error(e),
                        }
                        return MpvHandlerResponse::Ok;
                    }
                    return MpvHandlerResponse::Error("No song is playing".to_string());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Volume(id, volume) => {
                if let Some(state) = self.state.get_mut(&id) {
                    let controller = self.pulse_controller.borrow_mut();
                    let devices = controller
                        .list_devices()
                        .map_err(|e| e.to_string())
                        .map_err(|e| MpvHandlerResponse::Error(e));
                    let devices = match devices {
                        Ok(devices) => devices,
                        Err(e) => return e,
                    };
                    let device = devices.iter().find(|d| d.name == Some(id.clone()));

                    if let Some(dev) = device {
                        let pulse_volume =
                            Volume(((0x10000 as f64) * volume / 100.0).floor() as u32);
                        controller.set_device_volume_by_index(
                            dev.index,
                            &ChannelVolumes::default().set(2, pulse_volume),
                        );
                        state.volume = volume;
                        return MpvHandlerResponse::Ok;
                    } else {
                        return MpvHandlerResponse::Error(
                            "Device does not exist (setting volume)".to_string(),
                        );
                    }
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::GetVolume(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    return MpvHandlerResponse::Volume(state.volume);
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Add(id, song) => {
                if let Some(state) = self.state.get_mut(&id) {
                    state.queue.push(song);
                    //if the queue is now only one song, play it
                    if state.queue.len() == 1 {
                        let response = self.handle_message(MpvHandlerMessage::Play(id));
                        return response;
                    }
                    return MpvHandlerResponse::Ok;
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Remove(id, index) => {
                if let Some(state) = self.state.get_mut(&id) {
                    //removing index 0 is equivalent to stopping the song
                    if index == 0 {
                        let response = self.handle_message(MpvHandlerMessage::Stop(id));
                        return response;
                    }
                    if index < state.queue.len() {
                        state.queue.remove(index);
                        return MpvHandlerResponse::Ok;
                    }
                    return MpvHandlerResponse::Error("Index out of bounds".to_string());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::Clear(id) => {
                if let None = self.state.get(&id) {
                    return MpvHandlerResponse::Error("Device does not exist".to_string());
                }
                let response = self.handle_message(MpvHandlerMessage::Stop(id.clone()));
                if let MpvHandlerResponse::Ok = response {
                    if let Some(state) = self.state.get_mut(&id) {
                        state.queue.clear();
                    } else {
                        return MpvHandlerResponse::Error(
                            "Device does not exist clear 2nd half".to_string(),
                        );
                    }
                    return MpvHandlerResponse::Ok;
                }
                return response;
            }
            MpvHandlerMessage::List(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    return MpvHandlerResponse::List(state.queue.clone());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
            MpvHandlerMessage::TimeAndDuration(id) => {
                if let Some(state) = self.state.get_mut(&id) {
                    if let Some(mpv_sock) = state.mpv_sock.borrow_mut() {
                        let time = get_property(mpv_sock, "time-pos")
                            .map_err(|e| e.to_string())
                            .map_err(|e| MpvHandlerResponse::Error(e));
                        let time = match time {
                            Ok(time) => time,
                            Err(e) => return e,
                        };
                        let duration = get_property(mpv_sock, "duration")
                            .map_err(|e| e.to_string())
                            .map_err(|e| MpvHandlerResponse::Error(e));
                        let duration = match duration {
                            Ok(duration) => duration,
                            Err(e) => return e,
                        };
                        return MpvHandlerResponse::TimeAndDuration(
                            time.as_f64().unwrap_or(0.0),
                            duration.as_f64().unwrap_or(0.0),
                        );
                    }
                    return MpvHandlerResponse::Error("No song is playing".to_string());
                }
                return MpvHandlerResponse::Error("Device does not exist".to_string());
            }
        }
    }
}
