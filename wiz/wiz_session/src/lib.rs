mod out_stream;
mod parse;

use out_stream::OutStream;
pub use parse::ParseSession;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::time::{Duration, Instant};
use wizc_cli::{Config, ConfigExt};

#[derive(Debug, Default)]
pub struct Session {
    pub config: Config,
    pub parse_session: ParseSession,
    timers: BTreeMap<String, (Instant, Option<Duration>)>,
    errors: Vec<Box<dyn Error>>,
    pub out_stream: OutStream,
}

impl Session {
    pub fn new(config: Config) -> Self {
        Self {
            parse_session: Default::default(),
            timers: Default::default(),
            errors: Default::default(),
            out_stream: if config.quiet() {
                OutStream::void()
            } else {
                Default::default()
            },
            config,
        }
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
        writeln!(self.out_stream, "{}: {}ms", name, stop.as_millis()).unwrap();
        r
    }

    pub fn emit_error<E: 'static + Error>(&mut self, error: E) {
        self.errors.push(Box::new(error))
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn local_spell_book_root(&self) -> &Path {
        let p = self.config.input();
        if p.is_dir() {
            p
        } else {
            p.parent().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    #[test]
    fn test_start_stop() {
        let mut session = super::Session::default();
        session.start("foo");
        session.stop("foo");
        session.get_duration("foo").unwrap();
    }

    #[test]
    fn test_timer() {
        let mut session = super::Session::default();
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
        let mut session = super::Session::default();
        session.emit_error(E {});
        assert_eq!(session.has_error(), true);
    }
}
