use std::fs::File;
use std::path::Path;
use std::io::{Read, Result};
use log::{debug, error, warn, log_enabled, info, Level};

pub fn parse(path: &Path) -> Result<()> {
    info!("parsing the CSV file: '{}'", path.display());

    let mut csv_file = File::open(path)?;
    let mut csv_data = String::new();
    csv_file.read_to_string(&mut csv_data)?;

    csv_data.split("\n").for_each(
        | line | {
            let trimmed = line.trim();
            if trimmed.len() > 1 {
                let parts:Vec<&str> = trimmed.split(",").collect();
                if parts.len() == 3 {
                    info!("parts are {}", 3)
                } else {
                    //error!("")
                }
            }
        }
    );

    println!("{}", csv_data);

    Ok(())
}