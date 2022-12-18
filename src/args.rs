use clap::{Error, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// The event file to parse
    #[arg(short, long)]
    pub file: String,

    /// The event file to parse
    #[arg(short, long)]
    pub output: String

}

pub fn parse() -> Result<Args, Error> {
    return Args::try_parse();
}
