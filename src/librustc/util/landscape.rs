use std::fs::File;
use std::io::Write;

static mut DUMP_FILE: Option<File> = None;
pub struct StackDebug {
    s: &'static str,
}

impl StackDebug {
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

impl Drop for StackDebug {
    fn drop(&mut self) {
        unsafe{
            write!(DUMP_FILE.as_mut().unwrap(), "-{}\n", &self.s).unwrap();
        }
    }
}
