use std::error::Error;

// read rust source code file containing the content in line by line
pub struct RsCodeFile {
    pub content: Vec<String>,
}



impl RsCodeFile {

    pub fn new() -> RsCodeFile {
        RsCodeFile { content: Vec::new() }
    }

    pub fn read_file(&mut self, file_full_path: &str) -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string(file_full_path)?; 
        let temp_lines: Vec<&str> = content.split("\n").collect();
        self.content.reserve(temp_lines.len());
        for s in temp_lines {
            self.content.push(s.to_string());
        }
        Ok(())
    }

    pub fn print_content(&self) {
        for c in &self.content {
            println!("{}", c);
        }
    }
}