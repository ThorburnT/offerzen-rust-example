extern crate cfg_if;
extern crate wasm_bindgen;
extern crate csv_to_json;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use csv_to_json::csv_to_json;
use std::io::Cursor;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn c_to_j(csv: String) -> String {
    let mut data = Cursor::new(csv.as_bytes().to_owned());
    let mut buf: Vec<u8> = vec![];
    csv_to_json(&mut data, &mut buf);
    let json = String::from_utf8_lossy(&mut buf[..]);
    json.to_owned().to_string()
}
