macro_rules! log {
    ($($args:expr),*) => {
        eprintln!("libfirewall: {}", format!($($args),*))
    };
}

pub(crate) use log;
