pub mod args;
pub mod clock;
use clap::Parser;

#[tokio::main]
async fn main() {

    let args = args::Cmd::parse();

    match args {
        args::Cmd::Clock => {
            clock::clock().await;
        }
    }
}
