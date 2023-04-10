#[macro_export]
macro_rules! pang_core_info {
    ($($arg:tt)*) => {
        println!(concat!("\x1b[32mCore: ", $($arg)*, "\x1b[0m"));
    };
}

#[macro_export]
macro_rules! pang_core_warn {
    ($($arg:tt)*) => {
        println!(concat!("\x1b[33mCore: ", $($arg)*, "\x1b[0m"));
    };
}

#[macro_export]
macro_rules! pang_core_error {
    ($($arg:tt)*) => {
        println!(concat!("\x1b[31mCore: ", $($arg)*, "\x1b[0m"));
    };
}

#[macro_export]
macro_rules! pang_info {
    ($($arg:tt)*) => {
        println!(concat!("\x1b[32mApp: ", $($arg)*, "\x1b[0m"));
    };
}

#[macro_export]
macro_rules! pang_warn {
    ($($arg:tt)*) => {
        println!(concat!("\x1b[33mApp: ", $($arg)*, "\x1b[0m"));
    };
}

#[macro_export]
macro_rules! pang_error {
    ($($arg:tt)*) => {
        println!(concat!("\x1b[31mApp: ", $($arg)*, "\x1b[0m"));
    };
}