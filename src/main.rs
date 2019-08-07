extern crate reqwest;
mod map_services;
mod hashing;
mod utils;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn process_request(url: String, logname: String, imageprefix: String){
    utils::create_directories(); // check if directories exist
    let mut resp = reqwest::get(&url).expect("request failed"); // Send HTTP request to static map service
    let mut buffer: Vec<u8> = vec![];
    resp.copy_to(&mut buffer).expect("Failed to copy image data"); // Copy requested image data to buffer
    let hash_value = hashing::calculate_hash(&buffer); // Compute Hash of image
    // read previous hash:
    let mut last_hash: u64 = 0;
    let log_path = format!("./logs/{}", &logname);
    if Path::new(&log_path).exists(){
        let file = File::open(&log_path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        let last_line = lines.last(); // read last line of log
        last_hash = last_line.unwrap().split(',').collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
    }

    else{
        File::create(&log_path).expect("Failed to create log file");
    }

    if hash_value != last_hash{
        // Image is different from last hash !
        // Save image, log into file
        let mut out = File::create(format!("./imgs/{}_{}.jpg",imageprefix, hash_value)).expect("failed to create file");
        let mut pos = 0;
        while pos < buffer.len() {
            let bytes_written = out.write(&buffer[pos..]);
            pos += bytes_written.unwrap();
        }
        utils::write_log(log_path, hash_value);
    }

}

fn main() -> Result<(), Box<std::error::Error>> {

    let url = map_services::get_yandex_url("sat".to_string(),10.0,20.0,10.5,20.7);
    process_request(url, String::from("yandex.txt"),String::from("yandex1"));
    let url = map_services::get_bing_url("Aerial".to_string(),10.0,20.0,10.5,20.7, "BINGKEY".to_string());
    process_request(url, String::from("bing.txt"),String::from("bing1"));
    let url = map_services::get_google_url("satellite".to_string(),10.0,20.0,10.5,20.7, "GOOGLEKEY".to_string());
    process_request(url, String::from("google.txt"),String::from("google1"));
    Ok(())
}
