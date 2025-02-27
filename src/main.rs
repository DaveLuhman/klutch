use hidapi::HidApi;
use windows::{
    Media::Control::{GlobalSystemMediaTransportControlsSessionManager, GlobalSystemMediaTransportControlsSession},
    Foundation::IAsyncOperation,
};
use std::{thread, time::Duration};

fn main() {
    let api = HidApi::new().expect("Failed to initialize HIDAPI");

    // Replace with your foot pedal's Vendor ID (VID) and Product ID (PID)
    let vendor_id = 0x1523; // Update with actual VID
    let product_id = 0x255; // Update with actual PID

    // Open the HID device
    let device = api.open(vendor_id, product_id).expect("Failed to open foot pedal");

    println!("Listening for foot pedal input...");

    let mut buf = [0u8; 8]; // Adjust buffer size if necessary

    loop {
        match device.read(&mut buf) {
            Ok(_) => {
                match buf[0] {
                    0x02 => {
                        println!("Play/Pause Pressed");
                        send_media_command("play_pause");
                    }
                    0x04 => {
                        println!("Rewind Pressed");
                        send_media_command("rewind");
                    }
                    0x00 => {
                        println!("Fast Forward Pressed");
                        send_media_command("fast_forward");
                    }
                    _ => {}
                }
            }
            Err(err) => {
                eprintln!("Error reading HID: {}", err);
            }
        }

        thread::sleep(Duration::from_millis(100)); // Prevent high CPU usage
    }
}

fn send_media_command(command: &str) {
    let session_manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .expect("Failed to get media session manager")
        .get()
        .expect("Failed to retrieve session manager");

    let session: Option<GlobalSystemMediaTransportControlsSession> = session_manager.GetCurrentSession().ok();

    if let Some(media_session) = session {
        match command {
            "play_pause" => {
                media_session.TryTogglePlayPauseAsync().ok();
            }
            "rewind" => {
                // Seek backward (assuming -10s)
                let position = media_session.GetPlaybackInfo().unwrap().PlaybackPosition().unwrap();
                let new_position = position - Duration::from_secs(10);
                media_session.TryChangePlaybackPositionAsync(new_position.as_millis() as i64).ok();
            }
            "fast_forward" => {
                // Seek forward (+10s)
                let position = media_session.GetPlaybackInfo().unwrap().PlaybackPosition().unwrap();
                let new_position = position + Duration::from_secs(10);
                media_session.TryChangePlaybackPositionAsync(new_position.as_millis() as i64).ok();
            }
            _ => {}
        }
    } else {
        eprintln!("No active media session found.");
    }
}
