//! Dummy
#![stable(feature = "landscape", since = "1.3.0")]

use crate::fs::File;
use crate::io::Write;

static mut DUMP_FILE: Option<File> = None;
/// Dummy
#[derive(Debug)]
#[stable(feature = "landscape", since = "1.3.0")]
pub struct StackDebug {
    s: &'static str,
}

impl StackDebug {
    /// Dummy
    #[stable(feature = "landscape", since = "1.3.0")]
    pub fn new(s: &'static str) -> StackDebug {
        unsafe{
            if let None = DUMP_FILE {
                DUMP_FILE = Some(File::create("dump_file.txt").unwrap());
            }
            write!(DUMP_FILE.as_mut().unwrap(), "+{}\n", &s).unwrap();
        }
        StackDebug{s}
    }
}

#[stable(feature = "landscape", since = "1.3.0")]
impl Drop for StackDebug {
    fn drop(&mut self) {
        unsafe{
            write!(DUMP_FILE.as_mut().unwrap(), "-{}\n", &self.s).unwrap();
        }
    }
}
