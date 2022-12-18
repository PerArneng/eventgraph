mod csv;
mod args;


use std::path::Path;
use std::process;
use std::process::exit;
use log::{debug, error, warn, log_enabled, info, Level};

fn main() -> std::io::Result<()> {

    env_logger::init();
    info!("eventgraph is starting up");

    let args = args::parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

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