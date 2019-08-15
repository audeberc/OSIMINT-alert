extern crate reqwest;
mod hashing;
mod map_services;
mod utils;
use std::cmp;
use std::env;
use std::thread;

use chrono::prelude::*;
use std::time::Duration;

extern crate clokwerk;
use clokwerk::Interval::*;
use clokwerk::Scheduler;

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
        let next_call = String::from(format!(
            "{}D, {}H, {}m, {}s",
            j.frequency_days, j.frequency_hours, j.frequency_minutes, j.frequency_seconds
        ));
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
        let source = j.source;
        if source == "Wikimapia" || source == "OSM" {
            let fun = move || {
                let time: DateTime<Utc> = Utc::now();
                println!(
                    "{} Processing URL {}, {}, next call in {}",
                    time.to_string(),
                    &url,
                    &prefix,
                    &next_call
                );
                let mut new_image = false;
                if source == "Wikimapia" {
                    new_image = utils::process_wikimapia_json_request(&url, &prefix);
                }
                if source == "OSM" {
                    new_image = utils::process_OSM_json_request(&url, &prefix);
                }
                if new_image {
                    println!(
                        "{} ! New json data detected ! {}, {}, next call in {}",
                        time.to_string(),
                        &url,
                        &prefix,
                        &next_call
                    );
                }
            };
            job.run(fun);
        } else {
            let fun = move || {
                let time: DateTime<Utc> = Utc::now();
                println!(
                    "{} Processing URL {}, {}, next call in {}",
                    time.to_string(),
                    &url,
                    &prefix,
                    &next_call
                );
                let new_image = utils::process_image_request(&url, &prefix, &img_extension);
                if new_image {
                    println!(
                        "{} ! New image detected ! {}, {}, next call in {}",
                        time.to_string(),
                        &url,
                        &prefix,
                        &next_call
                    );
                }
            };
            job.run(fun);
        }
    }

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}
