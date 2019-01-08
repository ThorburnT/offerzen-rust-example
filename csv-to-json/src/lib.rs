extern crate serde;
#[macro_use]
extern crate serde_json;

use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{Read, Write};

pub fn csv_to_json<R: Read, W: Write>(rdr: &mut R, wrt: &mut W) {
    let mut csv_rdr = csv::Reader::from_reader(rdr);
    let mut out_write = serde_json::Serializer::new(wrt);
    let headers = csv_rdr.headers().unwrap().clone();
    let mut seq = out_write.serialize_seq(None).unwrap();
    let num_headers = headers.len();

    for result in csv_rdr.records() {
        let row = result.unwrap();
        let num_fields = row.len();
        assert_eq!(num_fields, num_headers);
        let mut obj: HashMap<String, Value> = HashMap::new();
        headers.iter().zip(row.iter()).for_each(|(header, field)| {
            obj.insert(header.to_string(), json!(field));
        });
        seq.serialize_element(&obj).unwrap();
    }
    seq.end().unwrap();
}
