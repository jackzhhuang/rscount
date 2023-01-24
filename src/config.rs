pub struct RsCountConfig {
    pub search_path: Option<String>,
    pub search_file: Option<String>,
}

impl RsCountConfig {
    pub fn new() -> RsCountConfig {
        RsCountConfig {
            search_path: None,
            search_file: None,
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

           _ => {
                panic!("unknown command: {}", command_name);
           }
        }
    }

    fn parse_one_command(&mut self, arg: &str) {
        let first_eq = arg.find("=").expect("please use = to assign a value to an command.");
        let command_name = &arg[0..first_eq];
        let value = &arg[first_eq + 1..arg.len()];

        self.set_command(command_name, value);
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