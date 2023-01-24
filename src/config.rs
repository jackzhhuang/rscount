pub struct RsCountConfig {
    pub search_path: String,
}

impl RsCountConfig {
    pub fn new() -> RsCountConfig {
        RsCountConfig {
            search_path: String::from("."),
        }
    }

    fn set_command(&mut self, command_name: &str, value: &str) {
        match command_name {
           "path" => {
                self.search_path = value.to_string();
                println!("set search path = {}", value);
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