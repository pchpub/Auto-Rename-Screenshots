#[macro_export]
macro_rules! info {
    ($($arg:expr),*) => {
        print!("[INFO]");
        $(
            print!(" {}", $arg);
        )*
        print!("\n");
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:expr),*) => {
        print!("[WARN]");
        $(
            print!(" {}", $arg);
        )*
        print!("\n");
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),*) => {
        print!("[ERROR]");
        $(
            print!(" {}", $arg);
        )*
        print!("\n");
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:expr),*) => {
        if *DEBUG_MODE.lock().await {
            print!("[DEBUG]");
            $(
                print!(" {}", $arg);
            )*
            print!("\n");
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($($arg:expr),*) => {
        print!("[OK]");
        $(
            print!(" {}", $arg);
        )*
        print!("\n");
    };
}
