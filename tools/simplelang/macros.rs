#[macro_export]
macro_rules! expect {
    ($tok:expr,$pat:pat,$fail:expr) => {{
        if !matches!($tok, $pat) {
            $fail($tok);
            false
        } else {
            true
        }
    }}; 
}

#[macro_export]
macro_rules! err {
    ($err_var:expr, $($arg:tt)+) => {
        $err_var = true;
        use ygen::Support::Colorize;
        eprintln!("{}: {}", "Error".red().bold(), format_args!($($arg)+))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        use ygen::Support::Colorize;
        eprintln!("{}: {}", "Warning".yellow().bold(), format_args!($($arg)+))
    };
}