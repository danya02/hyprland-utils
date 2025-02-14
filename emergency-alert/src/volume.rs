use std::collections::HashSet;

use anyhow::anyhow;
use libpulse_binding::volume::{Volume, VolumeLinear};
use pulsectl::controllers::{
    AppControl, DeviceControl, SinkController,
    types::{ApplicationInfo, DevState, DeviceInfo},
};

#[derive(Debug, Clone)]
pub struct VolumeConfigSnapshot {
    pub applications: Vec<ApplicationInfo>,
    pub devices: Vec<DeviceInfo>,

    /// True if there was any opportunity for existing apps to emit sound:
    /// i.e. that the active output device wasn't muted or at zero volume.
    pub audio_was_possible: bool,
}

pub fn snapshot_volume(handler: &mut SinkController) -> anyhow::Result<VolumeConfigSnapshot> {
    let mut audio_was_possible = true;
    let devices = handler.list_devices()?;

    // If there are more than one output device, then we'll be conservative and assume audio is possible.
    if devices.len() == 1 {
        let device = &devices[0];

        // If muted, no audio
        if device.mute {
            audio_was_possible = false;
        }

        // If device wasn't running (=actively playing)
        if device.state != DevState::Running {
            audio_was_possible = false;
        }

        // If volume is zero
        if device.volume.is_muted() {
            audio_was_possible = false;
        }
    }

    Ok(VolumeConfigSnapshot {
        applications: handler.list_applications()?,
        devices,
        audio_was_possible,
    })
}

pub fn boost_volume_of_apps_since(
    handler: &mut SinkController,
    since_snapshot: &VolumeConfigSnapshot,
) -> anyhow::Result<()> {
    let single_sink = since_snapshot
        .devices
        .first()
        .filter(|_| since_snapshot.devices.len() == 1);

    for app in handler.list_applications()? {
        // If we have an app now, but not in the snapshot, then it needs to be unmuted: it is being boosted.
        // Otherwise, if it's an app that has remained since the snapshot, then it needs to be muted.
        if since_snapshot
            .applications
            .iter()
            .any(|x| x.index == app.index)
        {
            // If there could have been audio previously, we need to keep it playing:
            // therefore we only change the app's volume, not its mute state.
            // (TODO: this seems to not work properly with Pipewire, so disabled for now)
            if since_snapshot.audio_was_possible && false {
                println!("Old volume was: {:?}", app.volume);
                let mut volume = app.volume;
                let mut app_volume_factor = VolumeLinear(1.0);

                // I also need to decrease the volume by whatever the volume was for the overall sink (if there is one).
                // That way, the final amplitude of the audio coming from this app should stay below its original volume.
                if let Some(sink) = single_sink {
                    app_volume_factor.0 -= VolumeLinear::from(sink.volume.avg()).0;
                }

                app_volume_factor.0 -= 1.0;

                println!(
                    "Volume factor: {} = {:?}",
                    app_volume_factor.0,
                    Volume::from(app_volume_factor)
                );

                volume
                    .scale(app_volume_factor.into())
                    .ok_or(anyhow!("Cannot scale volume?"))?;

                // No way to set volume from main API, so use introspect.
                // See impl of set_app_mute to check that this is correct.
                let op = handler
                    .handler
                    .introspect
                    .set_sink_volume_by_index(app.index, &volume, None);

                println!("Setting volume of app {} to {:?}", app.index, volume);

                handler.handler.wait_for_operation(op)?;
            } else {
                handler.set_app_mute(app.index, true)?;
            }
        } else {
            handler.set_app_mute(app.index, false)?;
        }
    }

    // Any apps that have been added since the snapshot need to be unmuted.
    for app in since_snapshot.applications.iter() {
        if handler.get_app_by_index(app.index).is_err() {
            handler.set_app_mute(app.index, false)?;
        }
    }

    for device in handler.list_devices().unwrap() {
        let mut max_volume = device.volume;
        let max_volume = max_volume
            .inc_clamp(device.base_volume, device.base_volume)
            .ok_or(anyhow!("Cannot increase volume?"))?;
        handler.set_device_volume_by_index(device.index, max_volume);
        handler.set_device_mute_by_index(device.index, false);
    }

    Ok(())
}

pub fn apply_snapshot(
    handler: &mut SinkController,
    snapshot: &VolumeConfigSnapshot,
) -> anyhow::Result<()> {
    for device in snapshot.devices.iter() {
        handler.set_device_volume_by_index(device.index, &device.volume);
        handler.set_device_mute_by_index(device.index, device.mute);
    }

    let current_apps: HashSet<_> = handler
        .list_applications()?
        .into_iter()
        .map(|x| x.index)
        .collect();
    for app in snapshot.applications.iter() {
        // If the app disappeared in the meantime, ignore it.
        if !current_apps.contains(&app.index) {
            continue;
        }
        handler.set_app_mute(app.index, app.mute)?;

        // No way to set volume from main API, so use introspect.
        // See impl of set_app_mute to check that this is correct.
        let op = handler
            .handler
            .introspect
            .set_sink_volume_by_index(app.index, &app.volume, None);
        handler.handler.wait_for_operation(op)?;
    }

    Ok(())
}
