extern crate csv_to_json;

use csv_to_json::csv_to_json;
use std::io::{stdin, stdout};

fn main() {
    let std_in = stdin();
    let std_out = stdout();
    let mut input = std_in.lock();
    let mut output = std_out.lock();
    csv_to_json(&mut input, &mut output);
}
