/// Macro for logging info messages with blue color
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("[{}] {}", "SUCCESS".green(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("[{}] {}", "INFO".blue(), format_args!($($arg)*))
    };
}

/// Macro for logging warning messages with yellow color
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("[{}] {}", "WARN".yellow(), format_args!($($arg)*))
    }
}

/// Macro for logging error messages with red color
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("[{}] {}", "ERROR".red(), format_args!($($arg)*))
    }
}