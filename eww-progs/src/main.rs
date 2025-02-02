pub mod args;
pub mod clock;
pub mod workspaces;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = args::Cmd::parse();

    match args {
        args::Cmd::Clock => {
            clock::clock().await;
        }
        args::Cmd::Workspaces => {
            workspaces::workspaces().await;
        }
    }
}
