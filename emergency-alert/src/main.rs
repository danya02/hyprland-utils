use clap::Parser;

pub mod music;
pub mod volume;

fn main() {
    let args = emergency_alert::args::MyArgs::parse();
    emergency_alert::run(&args).unwrap();
}
