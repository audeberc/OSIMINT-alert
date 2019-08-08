use std::path::Path;
use chrono::prelude::*;

use std::fs::OpenOptions;
use std::io::Write;
pub fn create_directories() {
    if !(Path::new("./logs/").exists()){
        std::fs::create_dir("./logs").expect("failed to create log dir");
    }
    if !(Path::new("./imgs/").exists()){
        std::fs::create_dir("./imgs").expect("failed to create imgs dir");
    }
}

pub fn write_log(log_path: String, hash_value: u64){
let mut file = OpenOptions::new()
.write(true)
.append(true)
.open(&log_path)
.unwrap();
let time: DateTime<Utc>= Utc::now();
if let Err(e) = writeln!(file, "{},{}", hash_value, time.to_string() ) {
eprintln!("Couldn't write to file: {}", e);};
}
