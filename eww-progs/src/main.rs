pub mod args;
pub mod clock;
pub mod proc;
mod volume;
pub mod workspaces;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = args::Cmd::parse();

    match args {
        args::Cmd::Clock => clock::clock(),
        args::Cmd::HyprWorkspaces => workspaces::workspaces(),
        args::Cmd::Volume => volume::volume(),
        args::Cmd::ProcCount => proc::proc_count(),
    }
}
