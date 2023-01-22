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
        self.content = content.split("\n").map(|s| -> String {s.to_string()}).collect();
        Ok(())
    }

    pub fn print_content(&self) {
        for c in &self.content {
            println!("{}", c);
        }
    }
}