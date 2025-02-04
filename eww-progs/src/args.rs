use clap::Parser;

#[derive(Debug, Parser)]
pub enum Cmd {
    #[command(name = "clock")]
    Clock,

    #[command(name = "hypr-workspaces")]
    HyprWorkspaces,

    #[command(name = "pulse-volume")]
    Volume,
}
