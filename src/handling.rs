use anyhow::{anyhow, Result};
use std::{
    env,
    process::{Command, Stdio},
};

use crate::watching::ChangeEvent;

pub trait EventHandler {
    fn handle(&self, event: ChangeEvent) -> Result<()>;
}

pub struct CommandEventHandler {
    command: Vec<String>,
    status: bool,
}

impl CommandEventHandler {
    pub fn new(command: Vec<String>, status: bool) -> Self {
        CommandEventHandler { command, status }
    }

    pub fn new_shell(command: Vec<String>, status: bool) -> Self {
        let shell = env::var("SHELL").unwrap_or("sh".to_owned());
        let cmd = vec![shell, "-c".to_owned(), command.join(" ")];
        Self::new(cmd, status)
    }
}

impl EventHandler for CommandEventHandler {
    fn handle(&self, event: ChangeEvent) -> Result<()> {
        log::debug!("{event}, command: {:?}", self.command);

        let main_command = self
            .command
            .first()
            .ok_or_else(|| anyhow!("empty command"))?;
        let mut cmd = Command::new(main_command)
            .args(&self.command[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

        let status = cmd.wait()?;
        if self.status {
            if let Some(code) = status.code() {
                println!("Exited with: {code}")
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_shell_handler_executes_correctly() {
        let handler = CommandEventHandler::new(vec!["ls".to_owned()], false);
        let event = ChangeEvent::new();
        handler.handle(event).expect("handle must succeed");
    }

    #[test]
    fn test_shell_handler_executes_correctly() {
        let handler = CommandEventHandler::new(vec!["echo".to_owned(), "hi".to_owned()], false);
        let event = ChangeEvent::new();
        handler.handle(event).expect("handle must succeed");
    }

    #[test]
    fn test_handler_fails_without_any_command() {
        let handler = CommandEventHandler::new(Vec::new(), false);
        let event = ChangeEvent::new();
        handler
            .handle(event)
            .expect_err("handle return error when command is empty");
    }
}
