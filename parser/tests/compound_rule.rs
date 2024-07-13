use chrono::NaiveDate;
use common::compound_rule::{CompoundRule, CompoundRuleSeq, NamedCompoundRule};
use parser::parsing::compound_rule::error::CompoundRuleParseError;

fn parse_named_rule(s: &str) -> Result<NamedCompoundRule, CompoundRuleParseError> {
    parser::parse_compound_rule(s)
}

fn parse_rule(content: &str) -> CompoundRule {
    parser::parse_compound_rule(format!("rule_name: {content}").as_str())
        .unwrap()
        .rule
}

#[test]
fn name_correct() {
    let parsed = parse_named_rule("rule_name: any arg1").unwrap();
     assert_eq!(parsed.name, "rule_name")
}

#[test]
fn name_incorrect() {
    let parsed = parse_named_rule(": any arg1");
    assert!(parsed.is_err())
}

#[test]
fn no_repeat() {
    let parsed = parse_rule("any arg1");
    assert_eq!(parsed.iterations, 1)
}

#[test]
fn repeat() {
    let parsed = parse_rule("repeat any arg1");
    assert_eq!(parsed.iterations, -1)
}

#[test]
fn no_every() {
    let parsed = parse_rule("any arg1");
    assert_eq!(parsed.every, 1)
}

#[test]
fn every_number() {
    let parsed = parse_rule("every 3 any arg1");
    assert_eq!(parsed.every, 3)
}

#[test]
fn every_incorrect() {
    let parsed = parse_named_rule("n: every b any arg1");
    assert!(parsed.is_err())
}

#[test]
fn every_incorrect2() {
    let parsed = parse_named_rule("n: every 0 any arg1");
    assert!(parsed.is_err())
}

#[test]
fn no_seq() {
    let parsed = parse_named_rule("arg1");
    assert!(parsed.is_err())
}

#[test]
fn seq_any() {
    let parsed = parse_rule("any arg1 arg2");
    assert_eq!(parsed.seq, CompoundRuleSeq::Any)
}

#[test]
fn seq_all() {
    let parsed = parse_rule("all arg1 arg2");
    assert_eq!(parsed.seq, CompoundRuleSeq::All)
}

#[test]
fn seq_seq() {
    let parsed = parse_rule("seq arg1 arg2");
    assert_eq!(parsed.seq, CompoundRuleSeq::Order)
}

#[test]
fn seq_streak_correct() {
    let parsed = parse_rule("streak 2 arg1 arg2");
    assert_eq!(parsed.seq, CompoundRuleSeq::Streak(2, vec!(NaiveDate::default(), NaiveDate::default())))
}

#[test]
fn seq_streak_incorrect() {
    let parsed = parse_named_rule("n: streak arg1 arg2");
    assert!(parsed.is_err())
}

#[test]
fn seq_streak_incorrect2() {
    let parsed = parse_named_rule("n: streak 0 arg1 arg2");
    assert!(parsed.is_err())
}

#[test]
fn arg() {
    let parsed = parse_rule("any arg");
    let expected = CompoundRule::new(
        1, 1, CompoundRuleSeq::Any, vec!("arg".to_owned())
    );
    assert_eq!(parsed, expected)
}

#[test]
fn no_args() {
    let parsed = parse_named_rule("n: any");
    assert!(parsed.is_err())
}

#[test]
fn args() {
    let parsed = parse_rule("any arg arg2 arg3");
    let expected = CompoundRule::new(
        1, 1, CompoundRuleSeq::Any,
        vec!("arg".to_owned(), "arg2".to_owned(), "arg3".to_owned())
    );
    assert_eq!(parsed, expected)
}

#[test]
fn incorrect_arg() {
    let parsed = parse_named_rule("n: any #");
    assert!(parsed.is_err())
}

#[test]
fn rule() {
    let parsed = parse_named_rule("ex_1: repeat every 3 streak 3 arg_1 arg_2 arg_3");
    assert!(parsed.is_ok())
}

#[test]
fn rule_incorrect() {
    let parsed = parse_named_rule("ex_1: repeat every 3 wrong_seq 3 arg_1 arg_2 arg_3");
    assert!(parsed.is_err())
}