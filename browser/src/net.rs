use anyhow::Error;

use crate::net::error::ResponseError;
use crate::net::model::{ApiResponse, GetCourseDataResponse, LoadGameResponse, SaveGamePayload};

mod error;
mod model;

pub(super) fn upload_game_state(server_url: &str, game_state: &str, player_registration_id: i32) -> anyhow::Result<()> {
    let url = format!("{server_url}/save_game");
    let payload = SaveGamePayload {
        player_registration_id,
        game_state: game_state.to_string()
    };
    let res = reqwest::blocking::Client::new()
        .post(url)
        .json(&payload)
        .send()?;
    if res.status().is_success() {
        Ok(())
    } else {
        Err(Error::from(ResponseError::new(res.status().as_u16())))
    }
}

pub(super) fn download_game_state(server_url: &str) -> anyhow::Result<Option<String>> {
    let url = format!("{server_url}/load_game");
    let response = reqwest::blocking::get(url)?
        .json::<ApiResponse<LoadGameResponse>>()?;
    if response.status_code != 200 {
        Err(ResponseError::new(response.status_code))?
    } else {
        let game_state = response.data
            .expect("load_game api contract ensures presence of data")
            .game_state;
        if game_state.is_empty() {
            Ok(None)
        } else {
            Ok(Some(game_state))
        }
    }
}

pub(super) fn download_simple_rules(server_url: &str) -> anyhow::Result<String> {
    Ok(download_course_data(server_url)?.course_gamification_rule_conditions)
}

pub(super) fn download_compound_rules(server_url: &str) -> anyhow::Result<String> {
    Ok(download_course_data(server_url)?.gamification_complex_rules)
}

pub(super) fn download_rule_results(server_url: &str) -> anyhow::Result<String> {
    Ok(download_course_data(server_url)?.gamification_rule_results)
}

fn download_course_data(server_url: &str) -> anyhow::Result<GetCourseDataResponse> {
    let url = format!("{server_url}/get_course_data");
    let response = reqwest::blocking::get(url)?
        .json::<ApiResponse<GetCourseDataResponse>>()?;
    if response.status_code != 200 {
        Err(ResponseError::new(response.status_code))?
    } else {
        Ok(response.data
            .expect("get_course_data api contract ensures presence of data"))
    }
}