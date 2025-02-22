use std::{fs::File, io::BufReader, path::Path};

use music::StreamWrapper;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

pub mod args;
mod music;
mod volume;

pub fn run(args: &args::MyArgs) -> anyhow::Result<()> {
    let mut handler = pulsectl::controllers::SinkController::create().unwrap();

    // Before opening a stream, make a snapshot.
    let snapshot = volume::snapshot_volume(&mut handler)
        .expect("Failed to capture volume snapshot, bailing early");

    // Then, open the stream and start playing the sound.
    // This will create a new application, so we can then boost the volume.
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    if let Err(why) = volume::boost_volume_of_apps_since(&mut handler, &snapshot) {
        eprintln!("Failed to boost volume of apps, restoring snapshot early: {why}");
        volume::apply_snapshot(&mut handler, &snapshot)
            .expect("Failed to restore snapshot -- volume may be inconsistent!");
    }

    if !args.do_not_protect {
        ctrlc::set_handler({
            let snapshot = snapshot.clone();
            move || {
                println!("Shutting down early, restoring snapshot");
                let mut handler = pulsectl::controllers::SinkController::create().unwrap();

                volume::apply_snapshot(&mut handler, &snapshot).unwrap();
                std::process::exit(0);
            }
        })
        .expect("cannot set ctrl-c handler");
    }

    if let Err(why) = play_audio(&args.music_file, &stream_handle) {
        eprintln!("Failed to play audio: {why}");
        eprintln!("Restoring snapshot anyway");
    }

    // Once the sound has finished, restore the snapshot.
    if let Err(why) = volume::apply_snapshot(&mut handler, &snapshot) {
        eprintln!("Failed to restore snapshot: {why}");
        eprintln!("Volume may be inconsistent!");
    }

    Ok(())
}

fn play_audio(file: &Path, stream_handle: &OutputStreamHandle) -> anyhow::Result<()> {
    // Load a sound from a file, using a path relative to Cargo.toml
    // let file = "/home/danya/.steam/steam/steamui/sounds/deck_ui_achievement_toast.wav";
    // let file = "/home/danya/.steam/steam/steamui/sounds/timer_expired_alarm.wav";
    //let file = "./ocean-sound-theme/ocean/stereo/theme-demo.oga";
    let file = BufReader::new(File::open(file)?);
    // Decode that sound file into a source
    let source = Decoder::new(file)?;

    let (wrapped_source, signal) = StreamWrapper::new(source.convert_samples());
    // Play the sound directly on the device
    stream_handle.play_raw(wrapped_source)?;

    signal.recv()?;

    Ok(())
}
