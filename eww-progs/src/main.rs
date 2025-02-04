pub mod args;
pub mod clock;
mod volume;
pub mod workspaces;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = args::Cmd::parse();

    match args {
        args::Cmd::Clock => clock::clock(),
        args::Cmd::HyprWorkspaces => workspaces::workspaces(),
        args::Cmd::Volume => volume::volume(),
    }
}
