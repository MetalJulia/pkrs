#[macro_export]
macro_rules! log_expensive {
    ($lvl:expr, $($elem:expr),*) => {
        if log_enabled!($lvl) {
            log!($lvl, $($elem, )*);
        }
    }
}
