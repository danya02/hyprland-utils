use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct MyArgs {
    /// The file to play after setting the volume
    pub music_file: PathBuf,
}
