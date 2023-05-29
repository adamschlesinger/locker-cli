#[macro_export]
macro_rules! defer {
    {$($e:expr;)+} => {
        $(let _deferral = $crate::defer::Deferral { expr: || $e };)+
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
