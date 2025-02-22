use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct MyArgs {
    /// The file to play after setting the volume
    pub music_file: PathBuf,

    /// Skip setting up protection against Ctrl+C being pressed in the middle of playback
    #[arg(long)]
    pub do_not_protect: bool,
}
