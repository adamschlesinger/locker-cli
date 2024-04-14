use std::time::SystemTime;
use chrono::{DateTime, Local, Timelike};

/// todo
#[macro_export]
macro_rules! defer {
    {$($e:expr;)+} => {
        $(let _deferral = $crate::utils::Deferral { expr: || $e };)+
    };
}

#[doc(hidden)]
pub struct Deferral<F: FnMut()> {
    pub expr: F,
}

impl<F: FnMut()> Drop for Deferral<F> {
    fn drop(&mut self) {
        (self.expr)();
    }
}

/// todo
pub fn timestamp() -> String {
    let now: DateTime<Local> = SystemTime::now().into();
    format!(
        "[{:0>2}:{:0>2}:{:0>2}]",
        now.hour(),
        now.minute(),
        now.second()
    )
}