
pub trait RsConfigProcessor {
    fn set_command(&mut self, command_name: &str, value: &str);
}

pub struct RsConfig<SomeConfig>
    where SomeConfig: RsConfigProcessor {
    config: SomeConfig,
}

impl<SomeConfig> RsConfig<SomeConfig> 
    where SomeConfig: RsConfigProcessor {

    pub fn new(s: SomeConfig) -> Self {
        RsConfig::<SomeConfig> {
            config: s,
        }
    }

    pub fn get(self) -> SomeConfig {
        self.config
    }

    fn parse_one_command(&mut self, arg: &str) {
        let first_eq = arg.find("=");
        if let None = &first_eq {
            self.config.set_command(arg, "1");
        } else {
            let eq_index = first_eq.unwrap();
            let command_name = &arg[0..eq_index];
            let value = &arg[eq_index + 1..arg.len()];

            self.config.set_command(command_name, value);
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