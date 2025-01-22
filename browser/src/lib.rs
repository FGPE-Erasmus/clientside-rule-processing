use processor::core::State;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::MakeWebConsoleWriter;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

mod net;
mod parsing;

#[wasm_bindgen]
pub async fn run(js_event: JsValue, server_url: &str, player_registration_id: i32) {
    let event = match serde_wasm_bindgen::from_value(js_event) {
        Ok(e) => e,
        Err(err) => {
            tracing::error!("could not deserialize event, aborting - details {err}");
            return;
        }
    };

    init_tracing();

    let potential_game_state = get_game_state(server_url).await;
    if let Err(err) = potential_game_state {
        tracing::error!("could not get game state, aborting - details {err}");
        return;
    }
    let mut game_state = potential_game_state.unwrap();
    let _ = game_state.update(&event);
    let saving_result = net::upload_game_state(
        game_state.save().as_str(), &game_state.save(), player_registration_id
    ).await;
    if let Err(err) = saving_result {
        tracing::error!("could not save game state, aborting - details {err}");
    }
}

async fn get_game_state(server_url: &str) -> anyhow::Result<State> {
    let state = if let Some(state_str) = net::download_game_state(server_url).await? {
        State::load(state_str.as_str())
    } else {
        State::new(
            parsing::parse_simple_rules(net::download_simple_rules(server_url).await?.as_str())?,
            parsing::parse_compound_rules(net::download_compound_rules(server_url).await?.as_str())?,
            parsing::parse_rule_results(net::download_rule_results(server_url).await?.as_str())?,
        )
    };
    Ok(state)
}

fn init_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .without_time()
        .with_writer(MakeWebConsoleWriter::new());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .init();
}