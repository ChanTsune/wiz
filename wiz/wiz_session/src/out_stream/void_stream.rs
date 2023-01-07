use std::io::{Result, Write};

pub(super) struct VoidStream;

impl Write for VoidStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
