pub mod rsfile;

use std::fmt::Error;

use rsfile::rs_code_file::RsCodeFile;

fn main() {
    let mut rs_file = RsCodeFile::new();
    let err = rs_file.read_file("/Users/jack/Documents/code/rust/vsrust/rscount/src/rsfile/rs_code_file.rs");
    if let Err(e) = err {
        panic!("error occurs: {:?}", e.to_string());
    }
    rs_file.print_content();
}
