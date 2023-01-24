use std::{path::Path, fs, io};
use super::rs_code_file::RsCodeFile;

pub struct RsCodeDir {
    pub total_line: u32,
}

impl RsCodeDir {
    pub fn new() -> RsCodeDir {
        RsCodeDir { total_line:0 }
    }
    
    pub fn process_rs_dir(&mut self, path_name: &str) -> io::Result<()> {
        let path = Path::new(path_name);

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path().canonicalize()?;
            let file_name = path.to_str().unwrap();
            if path.is_dir() {
                self.process_rs_dir(file_name);
            } else {
                if !file_name.ends_with(".rs") {
                    continue;
                }
                let mut rs_file = RsCodeFile::new();

                rs_file.process_rs_file(file_name);

                self.total_line += rs_file.rs_code_line;
            }
        }

        Ok(())
    }
}

