use super::config::RsConfigProcessor;

pub struct RsCountConfig {
    pub search_path: Option<String>,
    pub search_file: Option<String>,
    pub thread_pool: Option<String>,
}

impl RsCountConfig {
    pub fn new() -> RsCountConfig {
        RsCountConfig {
            search_path: None,
            search_file: None,
            thread_pool: None,
        }
    }
}

impl RsConfigProcessor for RsCountConfig {
    fn set_command(&mut self, command_name: &str, value: &str) {
        match command_name {
           "path" => {
                self.search_path = Some(value.to_string());
           } 

           "file" => {
                self.search_file = Some(value.to_string());
           }

           "thread" => {
                self.thread_pool = Some(value.to_string());
           }

           _ => {
                panic!("unknown command: {}", command_name);
           }
        }
    }
}