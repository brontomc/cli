
/// Macro for logging info messages with blue color
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("{} {}", "Done".green().bold(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", "Info".blue().bold(), format_args!($($arg)*))
    };
}

/// Macro for logging warning messages with yellow color
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("\n{} {}", "Warning".yellow().bold(), format_args!($($arg)*))
    }
}

/// Macro for logging error messages with red color
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{} {}", "Error".red().bold(), format_args!($($arg)*))
    }
}