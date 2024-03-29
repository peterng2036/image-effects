use std::string;

use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let bas64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&bas64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("DATA:IMGA/PNG;BASE64,{}", encoded_img);
    return data_url;
}
