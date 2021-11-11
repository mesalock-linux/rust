use crate::ffi::CStr;
use crate::io;
use crate::sys::{unsupported, Void};
use crate::time::Duration;
use crate::num::NonZeroUsize;

pub struct Thread(Void);

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    pub unsafe fn new(_stack: usize, _p: Box<dyn FnOnce()>) 
        -> io::Result<Thread>
    {
        unsupported()
    }

    pub fn yield_now() {
        panic!("unsupported")
    }

    pub fn set_name(_name: &CStr) {
        panic!("unsupported");
    }

    pub fn sleep(_dur: Duration) {
        panic!("unsupported");
    }

    pub fn join(self) {
        match self.0 {}
    }
}

pub fn available_concurrency() -> io::Result<NonZeroUsize> {
    unsupported()
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> { None }
    pub unsafe fn init() -> Option<Guard> { None }
}
