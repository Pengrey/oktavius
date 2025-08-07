use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "oktavius", author = "Pengrey", version = "1.0.0", about = "A tool to extract okta credentials from chromium")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    pub application: Application,

    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Application {
    #[value(alias = "m")]
    Msedge,
    #[value(alias = "c")]
    Chrome,
}

pub fn parse_args() -> Args {
    Args::parse()
}
