use chrono::Local;

#[derive(Default, Debug)]
pub(crate) struct Metrics {
    pub last_send_timestamp: i64,
}

impl Metrics {
    pub(crate) fn send_done(&mut self) {
        self.last_send_timestamp = Local::now().timestamp_millis();
    }
}
