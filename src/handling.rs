use std::process::{Command, Stdio};

use crate::watching::ChangeEvent;

pub trait EventHandler {
    fn handle(&self, event: ChangeEvent);
}

pub struct CommandEventHandler {
    command: Vec<String>,
    status: bool,
}

impl CommandEventHandler {
    pub fn new(command: Vec<String>, status: bool) -> Self {
        CommandEventHandler { command, status }
    }
}

impl EventHandler for CommandEventHandler {
    fn handle(&self, event: ChangeEvent) {
        log::debug!("{event}, command: {:?}", self.command);

        // FIXME this does not respect shell aliases, should it?
        let mut cmd = Command::new(
            self.command
                .first()
                .expect("command did not contain any elements"),
        )
        .args(&self.command[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

        let status = cmd.wait().unwrap();
        if self.status {
            if let Some(code) = status.code() {
                println!("Exited with: {code}")
            }
        }
    }
}
