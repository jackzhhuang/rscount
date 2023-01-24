mod rsfile;
mod config;

use rsfile::rs_code_dir::RsCodeDir;
use std::error::Error;

fn count_file(dir: &str) -> Result<(), Box<dyn Error>> {
    let mut rs_dir = RsCodeDir::new();
    rs_dir.process_rs_dir(dir)?;

    println!("total line is {}", rs_dir.total_line);

    Ok(())
}

fn accept_command() -> Result<(), Box<dyn Error>> {
    let mut config = config::RsCountConfig::new();
    config.parse_commands(&std::env::args().collect::<Vec<String>>());

    count_file(&config.search_path)?;

    Ok(())
}

fn main()-> Result<(), Box<dyn Error>> {
    accept_command()?;

    Ok(())
}