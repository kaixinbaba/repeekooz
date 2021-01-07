use crate::constants::{EventType, KeeperState};
use crate::ZKResult;

#[derive(Debug)]
pub struct WatchedEvent {
    pub keep_state: KeeperState,
    pub event_type: EventType,
    pub path: String,
}

pub trait Watcher {
    fn process(&self, event: WatchedEvent) -> ZKResult<()>;
}
