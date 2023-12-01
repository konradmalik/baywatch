use crate::watching::ChangeEvent;

pub trait EventHandler {
    fn handle(&self, event: ChangeEvent);
}

pub struct CommandEventHandler {
    command: Vec<String>,
}

impl CommandEventHandler {
    pub fn new(command: Vec<String>) -> Self {
        CommandEventHandler { command }
    }
}

impl EventHandler for CommandEventHandler {
    fn handle(&self, event: ChangeEvent) {
        log::debug!("{event}, command: {:?}", self.command);
    }
}
