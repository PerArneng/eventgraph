mod csv;

use clap::Parser;
use std::path::Path;
use std::process;
use log::{debug, error, warn, log_enabled, info, Level};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// The event file to parse
    #[arg(short, long)]
    file: String,

    /// The event file to parse
    #[arg(short, long)]
    output: String

}

fn main() -> std::io::Result<()> {

    env_logger::init();
    info!("eventgraph is starting up");

    let args = Args::parse();
    let path = Path::new(&args.file);
    if !path.exists() {
        error!("non existing file: '{}'", path.display());
        process::exit(1);
    }

    info!("reading from file '{}'", path.display());
    info!("writing output to '{}'", args.output);

    let result = csv::parse(&path);

    Ok(())
}