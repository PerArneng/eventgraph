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

}

fn main() {

    env_logger::init();
    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");

    let args = Args::parse();
    let path = Path::new(&args.file);
    if !path.exists() {
        error!("non existing file: {}!", path.display());
        process::exit(1);
    }

    info!("Hello {}!", path.display());
}