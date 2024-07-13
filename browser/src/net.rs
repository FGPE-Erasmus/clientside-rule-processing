use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::net::error::ResponseError;

mod error;

pub(super) fn upload_game_state(game_state: &str) -> anyhow::Result<()> {
    upload_data("url", game_state)
}

pub(super) fn download_game_state() -> anyhow::Result<Option<String>> {
    download_data("url")
}

pub(super) fn download_simple_rules() -> anyhow::Result<String> {
    Ok(download_data("url")?
        .expect("server-side simple rules str can not be empty"))
}

pub(super) fn download_compound_rules() -> anyhow::Result<String> {
    Ok(download_data("url")?
        .expect("server-side compound rules str can not be empty"))
}

pub(super) fn download_rule_results() -> anyhow::Result<String> {
    Ok(download_data("url")?
        .expect("server-side rule results str can not be empty"))
}

fn upload_data(url: &str, data: &str) -> anyhow::Result<()> {
    let res = reqwest::blocking::Client::new()
        .post(url)
        .json(data)
        .send()?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(Error::from(ResponseError::new(res.status())))
    }
}

fn download_data(url: &str) -> anyhow::Result<Option<String>> {
    let res = reqwest::blocking::get(url)?
        .json::<DataResponse<String>>()?;
    Ok(res.data)
}

#[derive(Serialize, Deserialize)]
struct DataResponse<T> {
    data: Option<T>
}