use std::collections::BTreeMap;
use std::error::Error;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Session {
    timers: BTreeMap<String, (Instant, Option<Duration>)>,
    errors: Vec<String>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            timers: Default::default(),
            errors: Default::default(),
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

    pub fn timer<T, F: FnOnce(&mut Self) -> T>(&mut self, name: &str, f: F) -> T {
        let start = Instant::now();
        let r = f(self);
        println!("{}: {}ms", name, start.elapsed().as_millis());
        r
    }

    pub fn emit_error<E: Error>(&mut self, error: E) {
        self.errors.push(error.to_string())
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
        session.timer("foo", |_| {
            println!("foo");
        });
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
