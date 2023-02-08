use std::{path::Path, fs, sync::Mutex, sync::Arc};
use super::rs_code_file::RsCodeFile;
use std::error::Error;
use rsthread_pool::rs_thread_pool::{RsThreadPool, RsCallBack};

pub struct RsThreadCallback {
    pub total_line: u32,
}

impl RsCallBack<RsCodeFile, String> for RsThreadCallback {
    fn callback(&mut self, t: &RsCodeFile) -> Result<(), Box<dyn Error>> {
        self.total_line += t.rs_code_line;
        Ok(())
    }
}

pub struct RsCodeDir {
    pub total_line: u32,
    thread_pool: Option<RsThreadPool<String>>,
    thread_callback: Arc<Mutex<RsThreadCallback>>,
}



impl RsCodeDir {
    pub fn new() -> RsCodeDir {
        RsCodeDir { 
            total_line:0,
            thread_pool: None, 
            thread_callback: Arc::new(Mutex::new(RsThreadCallback { total_line: 0 })),
        }
    }

    pub fn set_up_pool(&mut self) -> Result<(), Box<dyn Error>> {
        self.thread_pool = Some(RsThreadPool::<String>::new());
        self.thread_pool.as_mut().unwrap().set_up_pool(|| {
            RsCodeFile::new()
        }, &self.thread_callback)?;
        Ok(())
    }
    
    pub fn join(self) -> Result<(), Box<dyn Error>> {
        self.thread_pool.unwrap().join()?;
        println!("total line is {}", self.thread_callback.as_ref().lock().unwrap().total_line);
        Ok(())
    }

    pub fn process_rs_dir_in_multiple_thread(&mut self, path_name: &str) -> Result<(), Box<dyn Error>> {

        let path = Path::new(path_name);

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path().canonicalize()?;
            let file_name = path.to_str().unwrap();
            if path.is_dir() {
                self.process_rs_dir_in_multiple_thread(file_name)?;
            } else {
                if !file_name.ends_with(".rs") {
                    continue;
                }
                self.thread_pool.as_mut().unwrap().send(String::from(file_name))?;
            }
        }

        Ok(())
    }


    pub fn process_rs_dir(&mut self, path_name: &str) -> Result<(), Box<dyn Error>> {
        let path = Path::new(path_name);

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path().canonicalize()?;
            let file_name = path.to_str().unwrap();
            if path.is_dir() {
                self.process_rs_dir(file_name)?;
            } else {
                if !file_name.ends_with(".rs") {
                    continue;
                }
                let mut rs_file = RsCodeFile::new();

                rs_file.process_rs_file(file_name)?;

                self.total_line += rs_file.rs_code_line;
            }
        }

        Ok(())
    }
}

