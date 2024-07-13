use common::rule_result::{NamedRuleResult, RuleResult, RuleResultKind, RuleResultSeq, RuleResultValue};
use parser::parsing::rule_result::error::RuleResultParseError;

fn parse_named_res(s: &str) -> Result<NamedRuleResult, RuleResultParseError> {
    parser::parse_rule_result(s)
}

fn parse_res(content: &str) -> RuleResult {
    parser::parse_rule_result(format!("res_name -> {content}").as_str())
        .unwrap()
        .res
}

#[test]
fn name_correct() {
    let parsed = parse_named_res("res_name -> msg Hi").unwrap();
    assert_eq!(parsed.name, "res_name")
}

#[test]
fn name_incorrect() {
    let parsed = parse_named_res("# -> msg Hi");
    assert!(parsed.is_err())
}

#[test]
fn part_no_repeat() {
    let parsed = parse_res("msg Hi");
    assert_eq!(parsed.values.first().unwrap().iterations, 1);
}

#[test]
fn part_repeat() {
    let parsed = parse_res("repeat msg Hi");
    assert_eq!(parsed.values.first().unwrap().iterations, -1);
}

#[test]
fn part_incorrect_kind() {
    let parsed = parse_named_res("n -> cookie Hi");
    assert!(parsed.is_err());
}

#[test]
fn part_msg_kind() {
    let parsed = parse_res("msg Hi");
    assert_eq!(parsed.values.first().unwrap().kind, RuleResultKind::Message);
}

#[test]
fn part_reward_kind() {
    let parsed = parse_res("reward 1 pkt");
    assert_eq!(parsed.values.first().unwrap().kind, RuleResultKind::Reward);
}

#[test]
fn part_offer_kind() {
    let parsed = parse_res("offer 1 pkt special_badge");
    assert_eq!(parsed.values.first().unwrap().kind, RuleResultKind::Offer);
}

#[test]
fn part_open_kind() {
    let parsed = parse_res("open rule");
    assert_eq!(parsed.values.first().unwrap().kind, RuleResultKind::Open);
}

#[test]
fn part_restart_kind() {
    let parsed = parse_res("restart rule");
    assert_eq!(parsed.values.first().unwrap().kind, RuleResultKind::Restart);
}

#[test]
fn part_incorrect_seq() {
    let parsed = parse_named_res("n -> cookie Hi Hello");
    assert!(parsed.is_err());
}

#[test]
fn part_no_seq() {
    let parsed = parse_res("msg Hi Hello");
    assert_eq!(parsed.values.first().unwrap().seq, RuleResultSeq::All);
}

#[test]
fn part_all_seq() {
    let parsed = parse_res("msg all Hi Hello");
    assert_eq!(parsed.values.first().unwrap().seq, RuleResultSeq::All);
}

#[test]
fn part_seq_seq() {
    let parsed = parse_res("msg seq Hi Hello");
    assert_eq!(parsed.values.first().unwrap().seq, RuleResultSeq::Order);
}

#[test]
fn part_random_seq() {
    let parsed = parse_res("msg random Hi Hello");
    assert_eq!(parsed.values.first().unwrap().seq, RuleResultSeq::Random);
}

#[test]
fn part_random_once_seq() {
    let parsed = parse_res("msg random_once Hi Hello");
    assert_eq!(parsed.values.first().unwrap().seq, RuleResultSeq::RandomOnce);
}

#[test]
fn part_choice_seq() {
    let parsed = parse_res("msg choice Hi Hello");
    assert_eq!(parsed.values.first().unwrap().seq, RuleResultSeq::Choice);
}

#[test]
fn part_no_arg() {
    let parsed = parse_named_res("msg");
    assert!(parsed.is_err());
}

#[test]
fn part_incorrect_arg() {
    let parsed = parse_named_res("msg #");
    assert!(parsed.is_err());
}

#[test]
fn part_arg() {
    let parsed = parse_res("msg arg");
    assert_eq!(parsed.values.first().unwrap().values.first().unwrap(), "arg");
}

#[test]
fn part_args() {
    let parsed = parse_res("msg arg arg2 arg3");
    assert_eq!(parsed.values.first().unwrap().values, vec!("arg", "arg2", "arg3"));
}

#[test]
fn multiple_parts() {
    let parsed = parse_named_res("n -> msg Hi Hello; repeat offer random 1pkt badge emote");
    let expected = NamedRuleResult::new("n".to_owned(), RuleResult::new(vec!(
        RuleResultValue::new(1, RuleResultKind::Message, RuleResultSeq::All, vec!("Hi".to_owned(), "Hello".to_owned())),
        RuleResultValue::new(-1, RuleResultKind::Offer, RuleResultSeq::Random, vec!("1pkt".to_owned(), "badge".to_owned(), "emote".to_owned())),
    )));
    assert_eq!(parsed.unwrap(), expected)
}