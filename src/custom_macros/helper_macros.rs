//logging macro
macro_rules! logger {
    (error: $(arg:tt)*) => {
        println!("ERROR: {}", format!($($arg)*))
    };
    (info: $($arg:tt)*) => {
        println!("INFO: {}", format!($($arg)*))
    };
}

//error handler macro
macro_rules! handle_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}