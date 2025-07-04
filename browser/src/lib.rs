use std::sync::Once;
use serde::{Deserialize, Serialize};
use processor::core::State;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::MakeWebConsoleWriter;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use common::event::Event;
use common::rule_result::RuleResultKind;
mod parsing;

static INIT: Once = Once::new();

#[wasm_bindgen]
pub async fn process_event(js_input: JsValue) -> Option<JsValue> {
    INIT.call_once(|| {
        init_tracing();
    });

    let input: ProcessEventInput = match serde_wasm_bindgen::from_value(js_input) {
        Ok(i) => i,
        Err(err) => {
            tracing::error!("could not deserialize input, aborting - details {err}");
            return None;
        }
    };

    let mut state = if input.game_state.is_empty() {
        let sr = match parsing::parse_simple_rules(input.simple_rules_str.as_str()) {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("invalid simple rules string - details {err}");
                return None;
            }
        };
        let cr = match parsing::parse_compound_rules(input.compound_rules_str.as_str()) {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("invalid compound rule string - details {err}");
                return None;
            }
        };
        let rr = match parsing::parse_rule_results(input.rule_results_str.as_str()) {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("invalid rule results string - details {err}");
                return None;
            }
        };
        State::new(sr, cr, rr)
    } else {
        State::load(input.game_state.as_str())
    };

    let results = state.update(&input.event);

    let output = ProcessEventOutput {
        game_state: state.save(),
        results
    };

    Some(serde_wasm_bindgen::to_value(&output)
        .expect("should be able to convert output to JS value (api contract)"))
}

#[derive(Serialize, Deserialize)]
struct ProcessEventInput {
    event: Event,
    game_state: String,
    simple_rules_str: String,
    compound_rules_str: String,
    rule_results_str: String
}

#[derive(Serialize, Deserialize)]
struct ProcessEventOutput {
    game_state: String,
    results: Vec<(RuleResultKind, Vec<String>)>
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