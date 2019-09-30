//! Dummy
#![stable(feature = "stacktraceflow", since = "1.3.0")]

use crate::fs::File;
use crate::io::Write;

static mut DUMP_FILE: Option<File> = None;
/// Dummy
#[derive(Debug)]
#[stable(feature = "stacktraceflow", since = "1.3.0")]
pub struct StackTraceFlower {
    s: &'static str,
}

impl StackTraceFlower {
    /// Dummy
    #[stable(feature = "stacktraceflow", since = "1.3.0")]
    pub fn new(s: &'static str) -> StackTraceFlower {
        unsafe{
            if let None = DUMP_FILE {
                DUMP_FILE = Some(File::create("dump_file.txt").unwrap());
            }
            write!(DUMP_FILE.as_mut().unwrap(), "+{}\n", &s).unwrap();
        }
        StackTraceFlower{s}
    }
}

#[stable(feature = "stacktraceflow", since = "1.3.0")]
impl Drop for StackTraceFlower {
    fn drop(&mut self) {
        unsafe{
            write!(DUMP_FILE.as_mut().unwrap(), "-{}\n", &self.s).unwrap();
        }
    }
}
