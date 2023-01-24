mod rsfile;

use rsfile::rs_code_dir::RsCodeDir;

fn count_file(dir: &str) {
    let mut rs_dir = RsCodeDir::new();
    rs_dir.process_rs_dir(dir);

    println!("total line is {}", rs_dir.total_line);
}

fn main() {
    count_file("/Users/jack/Documents/code/rust/vsrust/rscount");
}