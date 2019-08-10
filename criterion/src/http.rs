use hyper::rt::{self, Future};
use hyper::Client;
//use std::io::{self, Write};

pub fn get() {
    rt::run(rt::lazy(|| {
        let client = Client::new();

        let uri = "http://httpbin.org/ip".parse().unwrap();

        client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status());
                println!("Headers: {:#?}", res.headers());
                println!("Body: {:#?}", res.body());
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));
}
