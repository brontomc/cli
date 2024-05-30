
/// Macro for logging info messages with blue color
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!("{} {}", " ✔ DONE ".on_green().bold(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{} {}", " i INFO ".on_blue().bold(), format_args!($($arg)*))
    };
}

/// Macro for logging warning messages with yellow color
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{} {}", " ▲ WARN ".on_yellow().bold(), format_args!($($arg)*))
    }
}

/// Macro for logging error messages with red color
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{} {}", " ✖ ERROR ".on_red().bold(), format_args!($($arg)*))
    }
}