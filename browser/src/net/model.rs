#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct ApiResponse<T> {
    pub(super) status_text: String,
    pub(super) status_code: u16,
    pub(super) data: Option<T>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct LoadGameResponse {
    pub(super) player_registration_id: i32,
    pub(super) game_state: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct GetCourseDataResponse {
    pub(super) course_gamification_rule_conditions: String,
    pub(super) gamification_complex_rules: String,
    pub(super) gamification_rule_results: String,
    pub(super) modules: Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(super) struct SaveGamePayload {
    pub(super) player_registration_id: i32,
    pub(super) game_state: String,
}