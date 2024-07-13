use std::str::FromStr;
use chrono::NaiveDate;
use common::{Advancing, AdvancingResultType};
use common::compound_rule::{CompoundRule, CompoundRuleSeq};

fn parse(s: &str) -> CompoundRule {
    parser::parse_compound_rule(s)
        .unwrap()
        .rule
}

fn default_arg_val() -> String {
    String::from_str("arg1").unwrap()
}

fn default_date_val() -> NaiveDate {
    NaiveDate::from_str("2000-01-01").unwrap()
}

fn extract_streak_vals(seq: CompoundRuleSeq) -> Vec<NaiveDate> {
    if let CompoundRuleSeq::Streak(_, vals) = seq {
        vals
    } else {
        unreachable!()
    }
}

//repeat

#[test]
fn no_repeat() {
    let mut parsed = parse("n: any arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    assert_eq!(adv.res_type, AdvancingResultType::Completed)
}

#[test]
fn repeat() {
    let mut parsed = parse("n: repeat any arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    assert_eq!(adv.res_type, AdvancingResultType::Restarted)
}

//every

#[test]
fn no_every() {
    let mut parsed = parse("n: any arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    assert_eq!(adv.res_type, AdvancingResultType::Completed)
}

#[test]
fn every_hit() {
    let mut parsed = parse("n: every 2 any arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    assert_eq!(adv.res_type, AdvancingResultType::Hit)
}

#[test]
fn every_completed() {
    let mut parsed = parse("n: every 2 any arg1 arg2");
    parsed.advance(&(&default_arg_val(), default_date_val()));
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    assert_eq!(adv.res_type, AdvancingResultType::Completed)
}

//any

#[test]
fn any() {
    let mut parsed = parse("n: any arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let expected_values = vec!("arg1".to_string(), "arg2".to_string());
    assert_eq!((adv.res_type, parsed.values), (AdvancingResultType::Completed, expected_values))
}

//all

#[test]
fn all_hit() {
    let mut parsed = parse("n: all arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let expected_values = vec!("arg2".to_string());
    assert_eq!((adv.res_type, parsed.values), (AdvancingResultType::Hit, expected_values))
}

#[test]
fn all_completed() {
    let mut parsed = parse("n: all arg1 arg2");
    parsed.advance(&(&"arg2".to_string(), default_date_val()));
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let expected_values = vec!("arg1".to_string());
    assert_eq!((adv.res_type, parsed.values), (AdvancingResultType::Completed, expected_values))
}

//seq

#[test]
fn seq_hit() {
    let mut parsed = parse("n: seq arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let expected_values = vec!("arg2".to_string());
    assert_eq!((adv.res_type, parsed.values), (AdvancingResultType::Hit, expected_values))
}

#[test]
fn seq_failed() {
    let mut parsed = parse("n: seq arg2 arg3");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let expected_values = vec!("arg2".to_string(), "arg3".to_string());
    assert_eq!((adv.res_type, parsed.values), (AdvancingResultType::None, expected_values))
}

#[test]
fn seq_completed() {
    let mut parsed = parse("n: seq arg1 arg2");
    parsed.advance(&(&default_arg_val(), default_date_val()));
    let adv = parsed.advance(&(&"arg2".to_string(), default_date_val()));
    let expected_values = vec!("arg2".to_string());
    assert_eq!((adv.res_type, parsed.values), (AdvancingResultType::Completed, expected_values))
}

//streak

#[test]
fn streak_one() {
    let mut parsed = parse("n: streak 1 arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let seq_vals = extract_streak_vals(parsed.seq);
    let expected_values = vec!("arg1".to_string(), "arg2".to_string());
    assert_eq!((adv.res_type, parsed.values, seq_vals),
               (AdvancingResultType::Completed, expected_values, vec!(NaiveDate::default())))
}

#[test]
fn streak_two_failed() {
    let mut parsed = parse("n: streak 2 arg1 arg2");
    parsed.advance(&(&default_arg_val(), default_date_val()));
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let seq_vals = extract_streak_vals(parsed.seq);
    let expected_values = vec!("arg1".to_string(), "arg2".to_string());
    assert_eq!((adv.res_type, parsed.values, seq_vals),
               (AdvancingResultType::Hit, expected_values, vec!(NaiveDate::from_str("2000-01-02").unwrap())))
}

#[test]
fn streak_two_completed() {
    let mut parsed = parse("n: streak 2 arg1 arg2");
    parsed.advance(&(&default_arg_val(), default_date_val()));
    let adv = parsed.advance(&(&default_arg_val(), NaiveDate::from_str("2000-01-02").unwrap()));
    let seq_vals = extract_streak_vals(parsed.seq);
    let expected_values = vec!("arg1".to_string(), "arg2".to_string());
    assert_eq!((adv.res_type, parsed.values, seq_vals),
               (AdvancingResultType::Completed, expected_values, vec!(NaiveDate::from_str("2000-01-02").unwrap())))
}

//rule

#[test]
fn rule_completed() {
    let mut parsed = parse("n: all arg1 arg2");
    parsed.advance(&(&default_arg_val(), default_date_val()));
    let adv = parsed.advance(&(&"arg2".to_string(), default_date_val()));
    let expected_values = vec!("arg2".to_string());
    assert_eq!((adv.res_type, parsed.values),
               (AdvancingResultType::Completed, expected_values))
}

#[test]
fn rule_hit() {
    let mut parsed = parse("n: seq arg1 arg2");
    let adv = parsed.advance(&(&default_arg_val(), default_date_val()));
    let expected_values = vec!("arg2".to_string());
    assert_eq!((adv.res_type, parsed.values),
               (AdvancingResultType::Hit, expected_values))
}