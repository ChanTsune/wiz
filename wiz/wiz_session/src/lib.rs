use std::collections::BTreeMap;
use std::error::Error;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Session {
    timers: BTreeMap<String, (Instant, Option<Duration>)>,
    errors: Vec<Box<dyn Error>>,
}

impl Session {
    pub fn new() -> Session {
        Session::default()
    }

    pub fn start(&mut self, id: &str) -> Instant {
        let now = Instant::now();
        self.timers.insert(id.to_string(), (now, None));
        now
    }

    pub fn stop(&mut self, id: &str) -> Duration {
        let now = Instant::now();
        let start = self.timers.get_mut(id);
        if let Some((start, duration)) = start {
            let time = now.duration_since(*start);
            duration.replace(time);
            return time;
        }
        Duration::default()
    }

    pub fn get_duration(&self, id: &str) -> Option<Duration> {
        match self.timers.get(id) {
            None => None,
            Some((_, duration)) => *duration,
        }
    }

    pub fn timer<T, F: FnOnce(&mut Self) -> T>(&mut self, name: &str, f: F) -> T {
        self.start(name);
        let r = f(self);
        let stop = self.stop(name);
        println!("{}: {}ms", name, stop.as_millis());
        r
    }

    pub fn emit_error<E: 'static + Error>(&mut self, error: E) {
        self.errors.push(Box::new(error))
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    #[test]
    fn test_start_stop() {
        let mut session = super::Session::new();
        session.start("foo");
        session.stop("foo");
        session.get_duration("foo").unwrap();
    }

    #[test]
    fn test_timer() {
        let mut session = super::Session::new();
        session.timer("foo", |_| {});
        session.get_duration("foo").unwrap();
    }

    #[test]
    fn test_has_errors() {
        #[derive(Debug)]
        struct E;
        impl Display for E {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str("")
            }
        }
        impl Error for E {}
        let mut session = super::Session::new();
        session.emit_error(E {});
        assert_eq!(session.has_error(), true);
    }
}
