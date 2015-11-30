extern crate hyper;

use std::convert::Into;
use std::io::{self, Read};
use hyper::client::Client;
use hyper::status::StatusCode;
use hyper::header::ContentType;

use self::Error::*;

pub struct Festivus {
    url: String
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Hyper(hyper::error::Error),
    Simple(String)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Io(e) }
}

impl From<hyper::error::Error> for Error {
    fn from(e: hyper::error::Error) -> Error { Hyper(e) }
}

impl Festivus {
    pub fn new<S: Into<String>>(s: S) -> Festivus {
        Festivus {
            url: Into::into(s)
        }
    }

    pub fn insert(&self, total: i32, hot_water: i32, solar: i32) -> Result<(), Error> {
        let client = Client::new();
        let url = format!("{}/power", self.url);
        let body = format!("total={}&hot_water={}&solar={}", total, hot_water, solar);
        let mut res = try!(
            client.post(&url).body(&body).header(ContentType::form_url_encoded()).send()
        );

        if res.status == StatusCode::Ok {
            Ok(())
        } else {
            let mut res_body = String::new();
            try!(res.read_to_string(&mut res_body));
            Err(Simple(format!("Error response from server: {}", res_body)))
        }
    }
}
