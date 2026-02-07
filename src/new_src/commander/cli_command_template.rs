use core::str;
use super::cli_command::CliCommand;

pub struct CliCommandTemplate{ 
    command_key: String,
    args: Vec<String>,
}



impl CliCommandTemplate { 
    fn new(command_key: &str, args: Vec<String>) -> Self { 
        CliCommandTemplate { 
            command_key: command_key.to_string(),
            args,
        }
    }
}



impl CliCommand for CliCommandTemplate { 

    fn assemble(self) -> String {
        self.command_key + self.args.join(" ").as_str()
    }
}
