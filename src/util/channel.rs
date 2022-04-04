use chrono::{Duration, Utc};
use crossbeam::channel::Receiver;
use log::debug;

pub trait CountReceiveChannel<T> {
    fn count_recvs_over_duration(&self, duration: Duration) -> i32;
    fn count_recvs_until_empty(&self) -> i64;
}


impl<T> CountReceiveChannel<T> for Receiver<T> {
    fn count_recvs_over_duration(&self, duration: Duration) -> i32 {
        let start = Utc::now();
        let mut counter = 0;
        while Utc::now() < start + duration {
            self.recv().unwrap();
            counter += 1;
        }
        return counter;
    }

    fn count_recvs_until_empty(&self) -> i64 {
        let mut counter = 0;
        while !self.is_empty() {
            self.recv().unwrap();
            counter += 1;
        }
        return counter;
    }
}
