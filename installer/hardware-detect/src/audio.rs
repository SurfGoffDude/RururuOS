use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
    pub server: AudioServer,
    pub devices: Vec<AudioDevice>,
    pub latency_capable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioServer {
    PipeWire,
    PulseAudio,
    Jack,
    Alsa,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub name: String,
    pub device_type: AudioDeviceType,
    pub is_default: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AudioDeviceType {
    Output,
    Input,
}

pub fn detect() -> AudioInfo {
    let server = detect_server();
    let devices = detect_devices(&server);
    let latency_capable = check_realtime_capable();
    
    AudioInfo {
        server,
        devices,
        latency_capable,
    }
}

fn detect_server() -> AudioServer {
    // Check PipeWire first
    if Command::new("pgrep").arg("pipewire").output()
        .map(|o| o.status.success()).unwrap_or(false) 
    {
        return AudioServer::PipeWire;
    }
    
    // Check PulseAudio
    if Command::new("pgrep").arg("pulseaudio").output()
        .map(|o| o.status.success()).unwrap_or(false) 
    {
        return AudioServer::PulseAudio;
    }
    
    // Check JACK
    if Command::new("pgrep").arg("jackd").output()
        .map(|o| o.status.success()).unwrap_or(false) 
    {
        return AudioServer::Jack;
    }
    
    // ALSA is always available on Linux
    if std::path::Path::new("/proc/asound").exists() {
        return AudioServer::Alsa;
    }
    
    AudioServer::None
}

fn detect_devices(server: &AudioServer) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    
    match server {
        AudioServer::PipeWire | AudioServer::PulseAudio => {
            // Use pactl
            if let Ok(output) = Command::new("pactl").args(["list", "sinks", "short"]).output() {
                let text = String::from_utf8_lossy(&output.stdout);
                for (i, line) in text.lines().enumerate() {
                    if let Some(name) = line.split('\t').nth(1) {
                        devices.push(AudioDevice {
                            name: name.to_string(),
                            device_type: AudioDeviceType::Output,
                            is_default: i == 0,
                        });
                    }
                }
            }
            
            if let Ok(output) = Command::new("pactl").args(["list", "sources", "short"]).output() {
                let text = String::from_utf8_lossy(&output.stdout);
                for (i, line) in text.lines().enumerate() {
                    if let Some(name) = line.split('\t').nth(1) {
                        if !name.contains(".monitor") {
                            devices.push(AudioDevice {
                                name: name.to_string(),
                                device_type: AudioDeviceType::Input,
                                is_default: i == 0,
                            });
                        }
                    }
                }
            }
        }
        AudioServer::Alsa => {
            // Parse /proc/asound/cards
            if let Ok(content) = std::fs::read_to_string("/proc/asound/cards") {
                for line in content.lines() {
                    if line.contains('[') && line.contains(']') {
                        if let Some(name) = line.split('[').nth(1).and_then(|s| s.split(']').next()) {
                            devices.push(AudioDevice {
                                name: name.trim().to_string(),
                                device_type: AudioDeviceType::Output,
                                is_default: devices.is_empty(),
                            });
                        }
                    }
                }
            }
        }
        _ => {}
    }
    
    devices
}

fn check_realtime_capable() -> bool {
    // Check if user is in audio group or has rtkit
    if let Ok(output) = Command::new("groups").output() {
        let groups = String::from_utf8_lossy(&output.stdout);
        if groups.contains("audio") || groups.contains("realtime") {
            return true;
        }
    }
    
    // Check rtkit
    Command::new("pgrep").arg("rtkit-daemon").output()
        .map(|o| o.status.success()).unwrap_or(false)
}
