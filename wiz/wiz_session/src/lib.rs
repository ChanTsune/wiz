use std::collections::BTreeMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Session {
    timers: BTreeMap<String, (Instant, Option<Duration>)>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            timers: Default::default(),
        }
    }

    pub fn start(&mut self, id: &str) {
        self.timers.insert(id.to_string(), (Instant::now(), None));
    }

    pub fn stop(&mut self, id: &str) {
        let now = Instant::now();
        let start = self.timers.get_mut(id);
        if let Some((start, duration)) = start {
            let time = now.duration_since(*start);
            duration.replace(time);
        }
    }

    pub fn get_duration(&self, id: &str) -> Option<Duration> {
        match self.timers.get(id) {
            None => None,
            Some((_, duration)) => duration.clone(),
        }
    }

    pub fn timer<T, F: FnOnce() -> T>(&self, name: &str, f: F) -> T {
        let start = Instant::now();
        let r = f();
        println!("{}: {}ms", name, start.elapsed().as_millis());
        r
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_start_stop() {
        let mut session = super::Session::new();
        session.start("foo");
        session.stop("foo");
        session.get_duration("foo").unwrap();
    }

    #[test]
    fn test_timer() {
        let session = super::Session::new();
        session.timer("foo", || {
            println!("foo");
        });
    }
}
