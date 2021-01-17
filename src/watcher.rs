use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;

use crate::constants::{EventType, KeeperState};
use crate::protocol::resp::WatcherEvent;
use crate::ZKResult;

/// ZooKeeper 回调通知对象
/// - `keep_state`： 服务端的状态，详细可见 [`KeeperState`]
/// - `event_type`： 事件类型，详细可见 [`EventType`]
/// - `path`： 触发事件的节点路径
#[derive(Debug)]
pub struct WatchedEvent {
    pub keep_state: KeeperState,
    pub event_type: EventType,
    pub path: String,
}

impl From<WatcherEvent> for WatchedEvent {
    fn from(server_event: WatcherEvent) -> Self {
        WatchedEvent {
            keep_state: KeeperState::from(server_event.keep_state as isize),
            event_type: EventType::from(server_event.event_type as isize),
            path: server_event.path,
        }
    }
}

/// 事件回调 trait，实现该 trait 即可自定义处理 ZooKeeper 回调通知
pub trait Watcher: Debug + Send {
    fn process(&self, event: &WatchedEvent) -> ZKResult<()>;
}

#[derive(Debug)]
pub(crate) struct WatcherManager {
    disable_auto_watch_reset: bool,
    data_watches: Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
    exists_watches: Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
    child_watches: Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
    persistent_watches: Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
    persistent_recursive_watches: Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
}

impl WatcherManager {
    pub(crate) fn register_data_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
    ) -> ZKResult<()> {
        self.register_watcher(path, watcher, &self.data_watches)
    }

    pub(crate) fn register_exists_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
    ) -> ZKResult<()> {
        self.register_watcher(path, watcher, &self.exists_watches)
    }

    fn register_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
        watches: &Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
    ) -> ZKResult<()> {
        let mut guard = watches.lock().unwrap();
        match guard.get_mut(&path) {
            Some(v) => v.push(watcher),
            _ => {
                let mut v = Vec::new();
                v.push(watcher);
                guard.insert(path, v);
            }
        }
        Ok(())
    }

    pub(crate) fn new(disable_auto_watch_reset: bool) -> Self {
        WatcherManager {
            disable_auto_watch_reset,
            data_watches: Mutex::new(HashMap::new()),
            exists_watches: Mutex::new(HashMap::new()),
            child_watches: Mutex::new(HashMap::new()),
            persistent_watches: Mutex::new(HashMap::new()),
            persistent_recursive_watches: Mutex::new(HashMap::new()),
        }
    }

    fn add_watches(
        &self,
        path: &String,
        watchers: &mut Vec<Box<dyn Watcher>>,
        result: &Mutex<HashMap<String, Vec<Box<dyn Watcher>>>>,
    ) {
        if let Some(mut v) = result.lock().unwrap().remove(path) {
            watchers.append(&mut v);
        }
    }

    fn add_persistent_watches(&self, path: &String, watchers: &mut Vec<Box<dyn Watcher>>) {
        if let Some(v) = self.persistent_watches.lock().unwrap().get_mut(path) {
            watchers.append(v);
        }
        // TODO 需要递归求出路径
        if let Some(v) = self
            .persistent_recursive_watches
            .lock()
            .unwrap()
            .get_mut(path)
        {
            watchers.append(v);
        }
    }

    pub(crate) fn find_need_triggered_watchers(
        &self,
        event: &WatchedEvent,
    ) -> Vec<Box<dyn Watcher>> {
        let mut watchers = Vec::new();
        match event.event_type {
            EventType::None => {
                let clear =
                    self.disable_auto_watch_reset && event.keep_state != KeeperState::SyncConnected;
                // data_watches
                for (_, v) in self.data_watches.lock().unwrap().iter_mut() {
                    watchers.append(v);
                }
                if clear {
                    self.data_watches.lock().unwrap().clear();
                }
                // exists_watches
                for (_, v) in self.exists_watches.lock().unwrap().iter_mut() {
                    watchers.append(v);
                }
                if clear {
                    self.exists_watches.lock().unwrap().clear();
                }
                // child_watches
                for (_, v) in self.child_watches.lock().unwrap().iter_mut() {
                    watchers.append(v);
                }
                if clear {
                    self.child_watches.lock().unwrap().clear();
                }
                // persistent_watches
                for (_, v) in self.persistent_watches.lock().unwrap().iter_mut() {
                    watchers.append(v);
                }
                // persistent_recursive_watches
                for (_, v) in self.persistent_recursive_watches.lock().unwrap().iter_mut() {
                    watchers.append(v);
                }
            }
            EventType::NodeCreated | EventType::NodeDataChanged => {
                self.add_watches(&event.path, &mut watchers, &self.data_watches);
                self.add_watches(&event.path, &mut watchers, &self.exists_watches);
                self.add_persistent_watches(&event.path, &mut watchers);
            }
            EventType::NodeChildrenChanged => {
                self.add_watches(&event.path, &mut watchers, &self.child_watches);
                self.add_persistent_watches(&event.path, &mut watchers);
            }
            EventType::NodeDeleted => {
                self.add_watches(&event.path, &mut watchers, &self.data_watches);
                self.add_watches(&event.path, &mut watchers, &self.exists_watches);
                self.add_watches(&event.path, &mut watchers, &self.child_watches);
                self.add_persistent_watches(&event.path, &mut watchers);
            }
            _ => panic!("Invalid EventType! {:?}", event.event_type),
        }
        watchers
    }
}
