
/// Macro for logging info messages with blue color
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("{} {}", " ✔ DONE:".green().bold(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", " i INFO:".blue().bold(), format_args!($($arg)*))
    };
}

/// Macro for logging warning messages with yellow color
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("\n{} {}", " ▲ WARN:".yellow().bold(), format_args!($($arg)*))
    }
}

/// Macro for logging error messages with red color
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{} {}", " ✖ ERROR:".red().bold(), format_args!($($arg)*))
    }
}