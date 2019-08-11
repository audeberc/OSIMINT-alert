extern crate reqwest;
mod hashing;
mod map_services;
mod utils;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cmp;
use std::env;
use std::thread;

use std::time::Duration;
use chrono::prelude::*;

extern crate clokwerk;
use clokwerk::Interval::*;
use clokwerk::Scheduler;

fn process_request(url: &String, site_name: &String, img_extension: &String) -> bool{
    utils::create_directories(); // check if directories exist
    let resp = reqwest::get(url);
    if !resp.is_err() {
        let mut resp_cont = resp.unwrap();
        let mut buffer: Vec<u8> = vec![];
        resp_cont.copy_to(&mut buffer)
            .expect("Failed to copy image data"); // Copy requested image data to buffer
        let hash_value = hashing::calculate_hash(&buffer); // Compute Hash of image
                                                           // read previous hash:
        let mut last_hash: u64 = 0;
        let log_path = format!("./logs/{}.txt", &site_name);
        if Path::new(&log_path).exists() {
            let file = File::open(&log_path).unwrap();
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
            let last_line = lines.last(); // read last line of log
            last_hash = last_line.unwrap().split(',').collect::<Vec<&str>>()[0]
                .parse::<u64>()
                .unwrap();
        } else {
            File::create(&log_path).expect("Failed to create log file");
        }

        if hash_value != last_hash {
            // Image is different from last hash !
            // Save image, log into file
            utils::save_image(site_name.to_string(), hash_value, img_extension.to_string(), buffer);
            utils::write_log(log_path, hash_value);
            true
        }
        else {
            false
        }
    }
    else {
        println!("Connection error: couldn't reach url");
        false
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut scheduler = Scheduler::new();
    let args: Vec<String> = env::args().collect();
    let conf_json_path = &args[1];
    let jobs_list: Vec<utils::Jobs> = utils::read_conf_json(conf_json_path.to_string());
    for j in jobs_list {
        let url_func = utils::get_url_function(&j);
        let img_extension = utils::get_img_extension(&j);
        let url = url_func(
            j.layer, j.lon_min, j.lat_min, j.lon_max, j.lat_max, j.api_key,
        );
        let prefix = String::from(format!("{}_{}", j.name, j.source));
        let next_call = String::from(format!("{}D, {}H, {}m, {}s", j.frequency_days,
                                             j.frequency_hours,
                                             j.frequency_minutes, j.frequency_seconds));
        let job = scheduler.every(Seconds(cmp::max(j.frequency_seconds, 1)));

        if j.frequency_hours != 0 {
            job.plus(Hours(j.frequency_hours));
        }
        if j.frequency_minutes != 0 {
            job.plus(Minutes(j.frequency_minutes));
        }
        if j.frequency_days != 0 {
            job.plus(Days(j.frequency_days));
        }

        let fun = move || {
            let time: DateTime<Utc> = Utc::now();
            println!("{} Processing URL {}, {}, next call in {}", time.to_string(), &url, &prefix, &next_call);
            let new_image = process_request(&url, &prefix, &img_extension);
            if new_image {
            println!("{} ! New image detected ! {}, {}, next call in {}", time.to_string(), &url, &prefix, &next_call);
            }
        };

        job.run(fun);
    }

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
