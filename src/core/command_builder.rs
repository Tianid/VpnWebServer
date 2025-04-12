use std::process::Command;
use crate::core::commands::command::CoreCommand;
use crate::core::commands::location::Location;
use crate::core::commands::restart_time::RestartTime;

pub struct CommandBuilder {
    base: String,
    command: Option<CoreCommand>
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self { base: "adguardvpn-cli".to_string(), command: None }
    }

    pub fn set_command(mut self, command: CoreCommand) -> Self {
        self.command = Some(command);
        self
    }

    pub fn build(&self) -> Command {
        if self.command.is_none() { return Command::new(self.base.as_str()) }

        match self.command.unwrap() {
            CoreCommand::Connect(location) => self.build_connect(location),
            CoreCommand::Disconnect        => self.build_disconnect(),
            CoreCommand::Restart(time)     => self.build_restart(time),
            CoreCommand::Status            => self.build_status()
        }
    }

    fn build_connect(&self, location: Location) -> Command {
        let mut command = Command::new(self.base.clone());
        command.arg("connect")
            .arg("-l")
            .arg(location.get_name());

        command
    }

    fn build_disconnect(&self) -> Command {
        let mut command = Command::new(self.base.clone());
        command.arg("disconnect");

        command
    }

    fn build_restart(&self, time: RestartTime) -> Command {
        let mut command = Command::new("shutdown");
        command.arg("-r");

        match time {
            RestartTime::Now                    => command.arg("now"),
            RestartTime::AfterSeconds(seconds)  => command.arg(seconds.to_string())
        };

        command
    }

    fn build_status(&self) -> Command {
        let mut command = Command::new(self.base.as_str());
        command.arg("status");

        command
    }
}
