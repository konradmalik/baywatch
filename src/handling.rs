use crate::watching::ChangeEvent;

pub fn handle_event(event: ChangeEvent) {
    log::debug!("{event}");
}
