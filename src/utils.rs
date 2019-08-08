use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

use crate::map_services;
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Jobs {
    pub name: String,
    pub source: String,
    pub layer: String,
    pub api_key: String,
    pub lat_min: f64,
    pub lat_max: f64,
    pub lon_min: f64,
    pub lon_max: f64,
    pub frequency_hours: u32,
    pub frequency_minutes: u32,
    pub frequency_seconds: u32,
    pub frequency_days: u32,
}

pub fn create_directories() {
    if !(Path::new("./logs/").exists()) {
        std::fs::create_dir("./logs").expect("failed to create log dir");
    }
    if !(Path::new("./imgs/").exists()) {
        std::fs::create_dir("./imgs").expect("failed to create imgs dir");
    }
}

pub fn write_log(log_path: String, hash_value: u64) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&log_path)
        .unwrap();
    let time: DateTime<Utc> = Utc::now();
    if let Err(e) = writeln!(file, "{},{}", hash_value, time.to_string()) {
        eprintln!("Couldn't write to file: {}", e);
    };
}

pub fn read_conf_json(json_path: String) -> Vec<Jobs> {
    let mut f = std::fs::File::open(json_path).expect("failed to load config json");
    let mut conf_string = String::new();
    f.read_to_string(&mut conf_string).unwrap();
    let v: Vec<Jobs> = serde_json::from_str(&conf_string).expect("cannot serialize json");
    v
}

pub fn get_url_function(
    job: &Jobs,
) -> fn(layer: String, lon0: f64, lat0: f64, lon1: f64, lat1: f64, key: String) -> String {
    match job.source.as_ref() {
        "Yandex" => map_services::get_yandex_url,
        "Google" => map_services::get_google_url,
        "Bing" => map_services::get_bing_url,
        _ => map_services::get_yandex_url,
    }
}

pub fn get_img_extension(job: &Jobs,) -> String {

    if job.source  == "Yandex"{
        match job.layer.as_ref() {
        "map" => "png".to_string(),
        _ => "jpg".to_string(),
        }
    }
    else {
        "jpg".to_string()
    }
}
