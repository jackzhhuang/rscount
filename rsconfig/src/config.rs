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

    fn parse_one_command(&mut self, arg: &str) {
        let first_eq = arg.find("=");
        if let None = &first_eq {
            self.set_command(arg, "1");
        } else {
            let eq_index = first_eq.unwrap();
            let command_name = &arg[0..eq_index];
            let value = &arg[eq_index + 1..arg.len()];

            self.set_command(command_name, value);
        }
    }

    pub fn parse_commands(&mut self, args: &[String]) {
        if args.len() == 0 {
            return;
        }

        for arg in &args[1..] {
            self.parse_one_command(arg);
        }
    }
}