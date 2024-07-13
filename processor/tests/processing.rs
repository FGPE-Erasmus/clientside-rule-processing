use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use common::event::{Event, EventPartValue};
use common::rule_result::RuleResultKind;
use processor::core::State;

fn default_event() -> Event {
    let parts = [
        ("player", EventPartValue::NumberBased(1)),
        ("did", EventPartValue::NumberBased(1)),
        ("with", EventPartValue::NumberBased(1)),
        ("in", EventPartValue::NumberBased(1)),
        ("of", EventPartValue::NumberBased(1)),
        ("on", EventPartValue::DateBased(NaiveDate::from_str("2000-01-01").unwrap())),
        ("at", EventPartValue::TimeBased(NaiveTime::from_str("10:00").unwrap())),
        ("achieving", EventPartValue::NumberBased(1)),
        ("repeat", EventPartValue::NumberBased(1))
    ].into_iter()
        .map(|(kw, val)| (kw.to_string(), val))
        .collect();
    Event::new(parts)
}

fn dump_simple_data(state: &State) -> (usize, usize) {
    (state.enabled_simple_rules.len(), state.disabled_simple_rules.len())
}

fn dump_compound_data(state: &State) -> (usize, usize) {
    (state.enabled_compound_rules.len(), state.disabled_compound_rules.len())
}

fn dump_res_data(state: &State) -> (usize, usize) {
    (state.enabled_rule_results.len(), state.disabled_rule_results.len())
}

//simple

#[test]
fn single_simple_hit() {
    let mut state = State::new(
        vec!(parser::parse_simple_rule("n: player 1 did 1 with all(1,2)").unwrap()),
        vec!(),
        vec!()
    );
    let data_after_init = dump_simple_data(&state);
    state.update(&default_event());
    let data_after_update = dump_simple_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (1, 0))
    )
}

#[test]
fn single_simple_completed() {
    let mut state = State::new(
        vec!(parser::parse_simple_rule("n: player 1 did 1 with 1").unwrap()),
        vec!(),
        vec!()
    );
    let data_after_init = dump_simple_data(&state);
    state.update(&default_event());
    let data_after_update = dump_simple_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (0, 1))
    )
}

#[test]
fn multiple_simple_completed() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("n: player 1 did 1 with all(1)").unwrap(),
            parser::parse_simple_rule("n2: on seq(2000.01.01) at 10:00").unwrap(),
            parser::parse_simple_rule("n3: player *").unwrap(),
        ),
        vec!(),
        vec!()
    );
    let data_after_init = dump_simple_data(&state);
    state.update(&default_event());
    let data_after_update = dump_simple_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((3, 0), (0, 3))
    )
}

#[test]
fn multiple_simple_mixed() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("n: player 3 did 1 with all(1)").unwrap(),
            parser::parse_simple_rule("n2: on seq(2000.01.01,2010.12.06) at 10:00").unwrap(),
            parser::parse_simple_rule("n3: player *").unwrap(),
        ),
        vec!(),
        vec!()
    );
    let data_after_init = dump_simple_data(&state);
    state.update(&default_event());
    let data_after_update = dump_simple_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((3, 0), (2, 1))
    )
}

//compound

#[test]
fn single_compound_hit() {
    let mut state = State::new(
        vec!(parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1,2)").unwrap()),
        vec!(parser::parse_compound_rule("compound_rule_1: repeat any simple_rule_1").unwrap()),
        vec!()
    );
    let data_after_init = dump_compound_data(&state);
    state.update(&default_event());
    let data_after_update = dump_compound_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (1, 0))
    )
}

#[test]
fn single_compound_completed() {
    let mut state = State::new(
        vec!(parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1)").unwrap()),
        vec!(parser::parse_compound_rule("compound_rule_1: any simple_rule_1").unwrap()),
        vec!()
    );
    let data_after_init = dump_compound_data(&state);
    state.update(&default_event());
    let data_after_update = dump_compound_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (0, 1))
    )
}

#[test]
fn multiple_compound_completed() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1)").unwrap(),
            parser::parse_simple_rule("simple_rule_2: on seq(2000.01.01) at 10:00").unwrap(),
            parser::parse_simple_rule("simple_rule_3: player *").unwrap(),
        ),
        vec!(
            parser::parse_compound_rule("compound_rule_1: any simple_rule_1").unwrap(),
            parser::parse_compound_rule("compound_rule_2: all simple_rule_1 simple_rule_2 simple_rule_3").unwrap(),
            parser::parse_compound_rule("compound_rule_3: every 1 all simple_rule_2").unwrap(),
        ),
        vec!()
    );
    let data_after_init = dump_compound_data(&state);
    state.update(&default_event());
    let data_after_update = dump_compound_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((3, 0), (0, 3))
    )
}

#[test]
fn multiple_compound_mixed() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1)").unwrap(),
            parser::parse_simple_rule("simple_rule_2: on seq(2000.01.02) at 10:00").unwrap(),
            parser::parse_simple_rule("simple_rule_3: player *").unwrap(),
        ),
        vec!(
            parser::parse_compound_rule("compound_rule_1: any simple_rule_1").unwrap(),
            parser::parse_compound_rule("compound_rule_2: all simple_rule_1 simple_rule_2 simple_rule_3").unwrap(),
            parser::parse_compound_rule("compound_rule_3: every 1 all simple_rule_2").unwrap(),
        ),
        vec!()
    );
    let data_after_init = dump_compound_data(&state);
    state.update(&default_event());
    let data_after_update = dump_compound_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((3, 0), (2, 1))
    )
}

//result

#[test]
fn single_res_hit() {
    let mut state = State::new(
        vec!(parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1,2)").unwrap()),
        vec!(parser::parse_compound_rule("compound_rule_1: repeat any simple_rule_1").unwrap()),
        vec!(parser::parse_rule_result("compound_rule_1 -> repeat msg Hi").unwrap())
    );
    let data_after_init = dump_res_data(&state);
    state.update(&default_event());
    let data_after_update = dump_res_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (1, 0))
    )
}

#[test]
fn single_res_completed() {
    let mut state = State::new(
        vec!(parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1)").unwrap()),
        vec!(parser::parse_compound_rule("compound_rule_1: any simple_rule_1").unwrap()),
        vec!(parser::parse_rule_result("compound_rule_1 -> msg Hi").unwrap())
    );
    let data_after_init = dump_res_data(&state);
    state.update(&default_event());
    let data_after_update = dump_res_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (0, 1))
    )
}

#[test]
fn multiple_res_completed() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1)").unwrap(),
            parser::parse_simple_rule("simple_rule_2: on seq(2000.01.01) at 10:00").unwrap(),
            parser::parse_simple_rule("simple_rule_3: player *").unwrap(),
        ),
        vec!(
            parser::parse_compound_rule("compound_rule_1: any simple_rule_1").unwrap(),
            parser::parse_compound_rule("compound_rule_2: all simple_rule_1 simple_rule_2 simple_rule_3").unwrap(),
            parser::parse_compound_rule("compound_rule_3: every 1 all simple_rule_2").unwrap(),
        ),
        vec!(
            parser::parse_rule_result("compound_rule_1 -> msg Hi").unwrap(),
            parser::parse_rule_result("compound_rule_2 -> reward random point badge emote cookie").unwrap(),
            parser::parse_rule_result("simple_rule_3 -> msg random_once Congratulations").unwrap()
        )
    );
    let data_after_init = dump_res_data(&state);
    state.update(&default_event());
    let data_after_update = dump_res_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((3, 0), (0, 3))
    )
}

#[test]
fn multiple_res_mixed() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule_1: player 1 did 1 with all(1,2)").unwrap(),
            parser::parse_simple_rule("simple_rule_2: on seq(2000.01.01) at 10:00").unwrap(),
            parser::parse_simple_rule("simple_rule_3: player *").unwrap(),
        ),
        vec!(
            parser::parse_compound_rule("compound_rule_1: any simple_rule_1").unwrap(),
            parser::parse_compound_rule("compound_rule_2: all simple_rule_1 simple_rule_2 simple_rule_3").unwrap(),
            parser::parse_compound_rule("compound_rule_3: every 1 all simple_rule_2").unwrap(),
        ),
        vec!(
            parser::parse_rule_result("compound_rule_1 -> msg Hi").unwrap(),
            parser::parse_rule_result("compound_rule_2 -> reward random point badge emote cookie").unwrap(),
            parser::parse_rule_result("simple_rule_3 -> msg random_once Congratulations").unwrap(),
            parser::parse_rule_result("compound_rule_3 -> reward all points badge").unwrap()
        )
    );
    let data_after_init = dump_res_data(&state);
    state.update(&default_event());
    let data_after_update = dump_res_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((4, 0), (2, 2))
    )
}

//special

#[test]
fn no_restart_res() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule: player 1 did 1 with all(1)").unwrap()
        ),
        vec!(
            parser::parse_compound_rule("compound_rule: any simple_rule").unwrap()
        ),
        vec!(
            parser::parse_rule_result("compound_rule -> msg Hi").unwrap()
        )
    );
    let data_after_init = dump_simple_data(&state);
    state.update(&default_event());
    let data_after_update = dump_simple_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (0, 1))
    )
}

#[test]
fn restart_res() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule: player 1 did 1 with all(1)").unwrap()
        ),
        vec!(
            parser::parse_compound_rule("compound_rule: any simple_rule").unwrap()
        ),
        vec!(
            parser::parse_rule_result("compound_rule -> restart simple_rule").unwrap()
        )
    );
    let data_after_init = dump_simple_data(&state);
    state.update(&default_event());
    let data_after_update = dump_simple_data(&state);
    assert_eq!(
        (data_after_init, data_after_update),
        ((1, 0), (1, 0))
    )
}

#[test]
fn res_restarting() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule: repeat + player 1 did 1 with all(1)").unwrap()
        ),
        vec!(
            parser::parse_compound_rule("compound_rule: repeat any simple_rule").unwrap()
        ),
        vec!(
            parser::parse_rule_result("compound_rule -> repeat msg Thanks").unwrap()
        )
    );
    let first_res = state.update(&default_event());
    let second_res = state.update(&default_event());
    let third_res = state.update(&default_event());
    assert_eq!(
        (first_res, second_res, third_res),
        (
            vec!((RuleResultKind::Message, vec!("Thanks".to_string()))),
            vec!((RuleResultKind::Message, vec!("Thanks".to_string()))),
            vec!((RuleResultKind::Message, vec!("Thanks".to_string())))
        )
    )
}

#[test]
fn res_non_restarting() {
    let mut state = State::new(
        vec!(
            parser::parse_simple_rule("simple_rule: player 1 did 1 with all(1)").unwrap()
        ),
        vec!(
            parser::parse_compound_rule("compound_rule: repeat any simple_rule").unwrap()
        ),
        vec!(
            parser::parse_rule_result("compound_rule -> repeat msg Thanks").unwrap()
        )
    );
    let first_res = state.update(&default_event());
    let second_res = state.update(&default_event());
    let third_res = state.update(&default_event());
    assert_eq!(
        (first_res, second_res, third_res),
        (
            vec!((RuleResultKind::Message, vec!("Thanks".to_string()))),
            vec!(),
            vec!()
        )
    )
}