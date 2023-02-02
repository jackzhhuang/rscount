use rsfile;
use rsconfig::config;

use rsfile::rs_code_dir::RsCodeDir;
use rsfile::rs_code_file::RsCodeFile;
use std::error::Error;

fn count_file(dir: &str) -> Result<(), Box<dyn Error>> {
    let mut rs_dir = RsCodeDir::new();
    rs_dir.process_rs_dir(dir)?;

    println!("total line in directroy({}) is {}", dir, rs_dir.total_line);

    Ok(())
}

fn accept_command() -> Result<Box<config::RsCountConfig>, Box<dyn Error>> {
    let mut config = Box::new(config::RsCountConfig::new());
    config.parse_commands(&std::env::args().collect::<Vec<String>>());

    Ok(config)

}

fn process_rsccount(config: Box<config::RsCountConfig>) -> Result<(), Box<dyn Error>> {
    if let Some(path_name) = &config.search_path {
        count_file(&path_name)?;
    }

    if let Some(file_name) = &config.search_file {
        let mut rs_file = RsCodeFile::new();
        rs_file.process_rs_file(file_name)?;

        println!("total line in file({}) is {}", file_name, rs_file.rs_code_line);
    }

    Ok(())
}

fn main()-> Result<(), Box<dyn Error>> {
    process_rsccount(accept_command()?).unwrap();

    Ok(())
}