#[macro_export]
macro_rules! log_response {
    ($endpoint:expr) => {
        log_ok!("Responded to /{} call successfully.", $endpoint)
    };
}

#[macro_export]
macro_rules! log_ok {
    ($($arg:tt)*) => {
        println!("[+] {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("[!] {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        eprintln!("[E] {}", format!($($arg)*))
    };
}
