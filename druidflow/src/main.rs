use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde_json::Value;
use opencv::{core, imgcodecs, prelude::*};

const API_KEY: &str = "YOUR_API_KEY";
const MODEL_ID: &str = "YOUR_MODEL_ID";

const INFERENCE_ENDPOINT: &str = "http://roboflow-inference-server:9001/";

fn detect_objects(frame: &Mat) -> Vec<Value> {
    let upload_url = format!(
        "{}{}?api_key={}",
        INFERENCE_ENDPOINT, MODEL_ID, API_KEY
    );
    let params: core::Vector<i32> = core::Vector::new();
    let mut buf = core::Vector::new();
    imgcodecs::imencode(".jpg", frame, &mut buf, &params).unwrap();

    let buf_vec: core::Vector<u8> = buf.into_iter().collect();
    let frame_str = STANDARD.encode(&buf_vec);

    let client = Client::new();
    let resp = client.post(&upload_url)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"))
        .body(frame_str)
        .send()
        .unwrap();

    let content = resp.text().unwrap();
    let result: Value = serde_json::from_str(&content).unwrap();
    let predictions: Vec<Value> = result["predictions"].as_array().unwrap().clone();

    predictions
}

fn main() {
    let img = imgcodecs::imread("test1.jpg", imgcodecs::IMREAD_COLOR).unwrap();
    let predictions = detect_objects(&img);
    println!("{:?}", predictions);
}