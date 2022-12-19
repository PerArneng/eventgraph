// some temporary allow's during
// initial development
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_variables)]

mod csv;
mod args;
mod logging;

use std::path::Path;
use std::process;
use std::process::exit;
use log::{info, error};

fn main() -> std::io::Result<()> {

    logging::setup();

    info!("Event Graph is starting up");

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