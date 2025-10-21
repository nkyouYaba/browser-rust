#![no_std]
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    let host = "http://example.com".to_string();
    let port = 8000;
    let path = "/".to_string();

    match client.get(host, port, path) {
        Ok(res) => {
            print!("response:\n{:#?}", res);
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }

    0
}

entry_point!(main);
