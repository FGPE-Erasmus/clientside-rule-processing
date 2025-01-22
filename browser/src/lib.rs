use common::event::Event;
use processor::core::State;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::MakeWebConsoleWriter;
use wasm_bindgen::prelude::wasm_bindgen;

mod net;
mod parsing;

#[wasm_bindgen]
pub fn run(event: Event, server_url: &str, player_registration_id: i32) {
    init_tracing();

    let potential_game_state = get_game_state(server_url);
    if let Err(err) = potential_game_state {
        tracing::error!("could not get game state, aborting - details {err}");
        return;
    }
    let mut game_state = potential_game_state.unwrap();
    let _ = game_state.update(&event);
    let saving_result = net::upload_game_state(
        game_state.save().as_str(), &game_state.save(), player_registration_id
    );
    if let Err(err) = saving_result {
        tracing::error!("could not save game state, aborting - details {err}");
    }
}

fn get_game_state(server_url: &str) -> anyhow::Result<State> {
    let state = if let Some(state_str) = net::download_game_state(server_url)? {
        State::load(state_str.as_str())
    } else {
        State::new(
            parsing::parse_simple_rules(net::download_simple_rules(server_url)?.as_str())?,
            parsing::parse_compound_rules(net::download_compound_rules(server_url)?.as_str())?,
            parsing::parse_rule_results(net::download_rule_results(server_url)?.as_str())?,
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