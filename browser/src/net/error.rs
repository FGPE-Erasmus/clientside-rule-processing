use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use reqwest::StatusCode;

pub struct ResponseError {
    status: StatusCode
}

impl ResponseError {
    pub fn new(status: StatusCode) -> Self {
        Self { status }
    }
}

impl Debug for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("response status code: {}", self.status).as_str())
    }
}

impl Error for ResponseError {
}