#[macro_export]
macro_rules! if_production_or {
	($prod_variant:expr, $non_prod_variant:expr) => {
		if cfg!(feature = "production") {
			$prod_variant
		} else {
			$non_prod_variant
		}
	};
}

#[macro_export]
macro_rules! if_not_production {
	($expression:expr) => {
		if cfg!(not(feature = "production")) {
			$expression
		}
	};
}

/// Logs a message at the error level. Silent for production build
#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => ($crate::if_not_production!(log::log!(target: $target, log::Level::Error, $($arg)+)));
    ($($arg:tt)+) => ($crate::if_not_production!(log::log!($log::Level::Error, $($arg)+)))
}

/// Logs a message at the warn level. Silent for production build
#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => ($crate::if_not_production!(log::log!(target: $target, log::Level::Warn, $($arg)+)));
    ($($arg:tt)+) => ($crate::if_not_production!(log::log!($log::Level::Warn, $($arg)+)))
}

/// Logs a message at the info level. Silent for production build
#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => ($crate::if_not_production!(log::log!(target: $target, log::Level::Info, $($arg)+)));
    ($($arg:tt)+) => ($crate::if_not_production!(log::log!(log::Level::Info, $($arg)+)))
}

/// Logs a message at the debug level. Silent for production build
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => ($crate::if_not_production!(log::log!(target: $target, log::Level::Debug, $($arg)+)));
    ($($arg:tt)+) => ($crate::if_not_production!(log::log!(log::Level::Debug, $($arg)+)))
}

/// Logs a message at the trace level. Silent for production build
#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => ($crate::if_not_production!(log::log!(target: $target, log::Level::Trace, $($arg)+)));
    ($($arg:tt)+) => ($crate::if_not_production!(log::log!(log::Level::Trace, $($arg)+)))
}
