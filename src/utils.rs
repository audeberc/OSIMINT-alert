use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
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
let time = SystemTime::now();
let since_the_epoch = time.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as u64;

if let Err(e) = writeln!(file, "{},{}", hash_value, since_the_epoch.to_string() ) {
eprintln!("Couldn't write to file: {}", e);};
}
