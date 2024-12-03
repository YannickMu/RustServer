use std::sync::Mutex;

pub static SHUTDOWN: Mutex<Vec<i32>> = Mutex::new(Vec::new());
