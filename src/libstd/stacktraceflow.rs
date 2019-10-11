//! Dummy
#![stable(feature = "stacktraceflow", since = "1.3.0")]

use crate::fs::File;
use crate::io::Write;
use crate::thread;
use crate::cell::RefCell;

thread_local! {
static DUMP_FILE: RefCell<Option<File>> = RefCell::new(None);
}

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
        DUMP_FILE.with(|f| {
            let mut f_ref = f.borrow_mut();
            if let None = *f_ref {
                let filename = format!("stacktraceflow.{:?}.txt", thread::current().id());
                *f_ref = Some(File::create(&filename).unwrap());
            }
            write!(f_ref.as_mut().unwrap(), "+{}\n", &s).unwrap();
        });
        StackTraceFlower{s}
    }
}

#[stable(feature = "stacktraceflow", since = "1.3.0")]
impl Drop for StackTraceFlower {
    fn drop(&mut self) {
        DUMP_FILE.with(|f| {
            write!(f.borrow_mut().as_mut().unwrap(), "-{}\n", &self.s).unwrap();
        });
    }
}
