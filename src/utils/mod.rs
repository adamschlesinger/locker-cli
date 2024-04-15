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
    let now: DateTime<Local> = Local::now();
    format!("[{:}]", now.format("%H:%M:%S%.3f"))
}