use std::fmt;
use std::fmt::{Arguments, Debug, Formatter, Pointer};
use std::io::{stdout, Write};

pub struct OutStream(Box<dyn Write>);

impl Debug for OutStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for OutStream
where
    T: Write + 'static,
{
    fn from(write: T) -> Self {
        Self(Box::new(write))
    }
}

impl Default for OutStream {
    fn default() -> Self {
        Self::from(stdout())
    }
}

impl fmt::Write for OutStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0
            .write(s.as_bytes())
            .map(|_| ())
            .map_err(|_| fmt::Error::default())
    }
}
