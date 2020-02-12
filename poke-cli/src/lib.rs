use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "poke")]
pub struct App {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(name = "web")]
    Web {
        #[structopt(
            long = "port",
            short = "p",
            required = true,
            default_value = "3030",
            help = "http port to use for accepting connections"
        )]
        port: u16,
    },
}

// Use of a mod or pub mod is not actually necessary.
mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
