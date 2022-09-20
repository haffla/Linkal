use serde_json;
use serde_json::{Map, Value};
use std::fs;
use warp::reject::Reject;
mod filters;
mod handlers;

#[derive(Debug)]
struct ConversionError;
impl Reject for ConversionError {}

#[derive(Debug)]
pub struct Calendar {
    name: String,
    url: String,
    path: String,
}

#[tokio::main]
async fn main() {
    let path = "../calendars.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).unwrap();
    let cals = res["calendars"].clone();
    let obj: Map<String, Value> = cals.as_object().unwrap().clone();
    let mut calendars: Vec<Calendar> = Vec::new();
    for (k, v) in obj {
        let split = k.clone();
        let splitted = split.split("/");
        let vec: Vec<&str> = splitted.collect();
        calendars.push(Calendar {
            name: String::from(v["name"].as_str().unwrap_or("Unamed calendar")),
            url: String::from(k),
            path: String::from(vec[vec.len() - 1]),
        });
    }
    pretty_env_logger::init();
    let routes = filters::api(calendars);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
