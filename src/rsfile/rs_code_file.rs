use std::{path::Path, error::Error, fs, io, iter::Iterator};

// read rust source code file containing the content in line by line
pub struct RsCodeFile {
    pub rs_code_line: u32,
    waiting_comment_end: bool
}

/*
this is a test comment println!("\*\/")
*/  // this is annother
impl RsCodeFile {

    pub fn new() -> RsCodeFile {
        RsCodeFile { 
            rs_code_line: 0, 
            waiting_comment_end: false
        }
    }

    pub fn process_rs_file(&mut self, file_full_path: &str) -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string(file_full_path)?; 
        content.split("\n").for_each(|s| {
            self.process_each_line(s);
        });
        Ok(())
    }

    fn check_line_left(&mut self, trim_line: &str) {
        // skip the line which begins with //
        if trim_line.starts_with("//") {
            return;
        }
        
        // the line which begins with /*
        if trim_line.starts_with("/*") {
            self.waiting_comment_end = true;
            if trim_line.ends_with("*/") { // ends with */
                self.waiting_comment_end = false;
            }
            return;
        }
        
        // not comment or space line, count it
        self.rs_code_line += 1;
    }

    fn skip_comment(&mut self, trim_line: &str) {
        // to see if in the state waiting for */ to end the comment
        if self.waiting_comment_end {
            if trim_line.contains("*/") {
                // the line contains */ so the comment ends
                self.waiting_comment_end = false;
            }
            if trim_line.ends_with("*/") {
                return; // this line will be skipped
            } else {
                // the line dose not end with */
                // skip the content before */
                let end = trim_line.find("*/");
                if let Some(mut index) = end {
                    index += 2; // skip */

                    // check the content after */ which is the part of the source code
                    self.check_line_left((&trim_line[index..trim_line.len()].trim()));
                }
            }         
        } else {
            // not the state, check the line
            self.check_line_left(trim_line);
        }
    }

    // check each line:
    // 1, trim the space
    // 2, if in the state waiting for the end of comment, skip the comment
    // 3, if there are other contents left behind the comment, check the left
    fn process_each_line(&mut self, line: &str) {
        let trim_line = line.trim();

        if trim_line.len() == 0 {
            return;
        }

        self.skip_comment(trim_line);
    }
}