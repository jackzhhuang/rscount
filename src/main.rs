pub mod rsfile;

use std::fmt::Error;

use rsfile::rs_code_file::RsCodeFile;

fn main() {
    let file_name = "/Users/jack/Documents/code/rust/vsrust/rscount/src/rsfile/rs_code_file.rs";
    let mut rs_file = RsCodeFile::new();
    let err = rs_file.process_rs_file(file_name);
    if let Err(e) = err {
        panic!("error occurs: {:?}", e.to_string());
    }

    println!("the line of {} is {}", file_name, rs_file.rs_code_line);
}
