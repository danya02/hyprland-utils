use pulsectl::controllers::DeviceControl;
use serde::Serialize;

#[derive(Serialize)]
struct Volume {
    volume: f64,
    is_muted: bool,
}

pub fn volume() -> anyhow::Result<()> {
    let mut sinks = pulsectl::controllers::SinkController::create()?;
    loop {
        let device = sinks.get_default_device()?;

        let volume_frac = device.volume.avg().0 as f64 / device.base_volume.0 as f64;

        let volume = Volume {
            volume: volume_frac,
            is_muted: device.mute,
        };
        println!("{}", serde_json::to_string(&volume).unwrap());

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
