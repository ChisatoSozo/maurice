use paperclip::actix::Apiv2Schema;
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Apiv2Schema)]
pub struct AudioDevice {
    pub name: String,
}

pub fn get_audio_devices() -> Result<Vec<AudioDevice>, Box<dyn std::error::Error>> {
    let mut handler = SinkController::create().unwrap();

    let devices = handler.list_devices()?;

    let mut audio_devices = Vec::new();
    for device in devices {
        audio_devices.push(AudioDevice {
            name: device.name.unwrap_or(device.index.to_string()),
        });
    }

    Ok(audio_devices)
}

#[test]
fn test_list_audio_devices() {
    let devices = get_audio_devices().expect("Failed to get audio devices");
    assert!(!devices.is_empty(), "No audio devices found");
    for device in devices {
        println!("Found device: {}", device.name);
        assert!(!device.name.is_empty(), "Device name should not be empty");
    }
}
