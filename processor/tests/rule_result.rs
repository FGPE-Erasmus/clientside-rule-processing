use common::{Advancing, AdvancingResultType};
use common::rule_result::{RuleResult, RuleResultKind};

fn parse(s: &str) -> RuleResult {
    parser::parse_rule_result(s)
        .unwrap()
        .res
}

fn single_data(kind: RuleResultKind, args: Vec<String>) -> Option<Vec<(RuleResultKind, Vec<String>)>> {
    Some(vec!((kind, args)))
}

//repeat

#[test]
fn no_repeat() {
    let mut parsed = parse("n -> msg Hi");
    let adv = parsed.advance(&());
    assert_eq!(adv.res_type, AdvancingResultType::Completed)
}

#[test]
fn repeat() {
    let mut parsed = parse("n -> repeat msg Hi");
    let adv = parsed.advance(&());
    assert_eq!(adv.res_type, AdvancingResultType::Hit)
}

//msg

#[test]
fn msg() {
    let mut parsed = parse("n -> msg Hi");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Message, vec!("Hi".to_string()))))
}

#[test]
fn msg_two() {
    let mut parsed = parse("n -> msg Hi Hello");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Message, vec!("Hi".to_string(), "Hello".to_string()))))
}

//reward

#[test]
fn reward() {
    let mut parsed = parse("n -> reward seq 1 pkt");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Reward, vec!("1".to_string(), "pkt".to_string()))))
}

#[test]
fn reward_two() {
    let mut parsed = parse("n -> reward seq 1 pkt 1 badge");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Hit,
                single_data(RuleResultKind::Reward,
                            vec!("1".to_string(), "pkt".to_string())))
    )
}

//offer

#[test]
fn offer() {
    let mut parsed = parse("n -> offer seq pkt badge cookie");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed,
                single_data(RuleResultKind::Offer,
                            vec!("pkt".to_string(), "badge".to_string(), "cookie".to_string())))
    )
}

#[test]
fn offer_two() {
    let mut parsed = parse("n -> offer seq pkt badge cookie time emote image");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Hit,
                single_data(RuleResultKind::Offer,
                            vec!("pkt".to_string(),"badge".to_string(), "cookie".to_string())))
    )
}

//open

#[test]
fn open() {
    let mut parsed = parse("n -> open seq ex1");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Open, vec!("ex1".to_string()))))
}

#[test]
fn open_two() {
    let mut parsed = parse("n -> open seq ex1 ex2");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Hit, single_data(RuleResultKind::Open, vec!("ex1".to_string()))))
}

//restart

#[test]
fn restart() {
    let mut parsed = parse("n -> restart seq rule_1");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Restart, vec!("rule_1".to_string()))))
}

#[test]
fn restart_two() {
    let mut parsed = parse("n -> restart seq rule_1 rule_2");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Hit, single_data(RuleResultKind::Restart, vec!("rule_1".to_string()))))
}

//all

#[test]
fn all() {
    let mut parsed = parse("n -> msg all Hi Hello");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Message, vec!("Hi".to_string(), "Hello".to_string()))))
}

//seq

#[test]
fn seq_hit() {
    let mut parsed = parse("n -> msg seq Hi Hello");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Hit, single_data(RuleResultKind::Message, vec!("Hi".to_string()))))
}

#[test]
fn seq_completed() {
    let mut parsed = parse("n -> msg seq Hi Hello");
    parsed.advance(&());
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Message, vec!("Hello".to_string()))))
}

//random_once

#[test]
fn random_once_hit() {
    let mut parsed = parse("n -> msg random_once Hi Hello");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data.unwrap().first().unwrap().1.len()),
               (AdvancingResultType::Hit, 1))
}

#[test]
fn random_once_completed() {
    let mut parsed = parse("n -> msg random_once Hi Hello");
    parsed.advance(&());
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data.unwrap().first().unwrap().1.len()),
               (AdvancingResultType::Completed, 1))
}

//random

#[test]
fn random() {
    let mut parsed = parse("n -> msg random Hi Hello");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data.unwrap().first().unwrap().1.len()),
               (AdvancingResultType::Completed, 1))
}

//choice

#[test]
fn choice() {
    let mut parsed = parse("n -> msg choice Hi Hello");
    parsed.advance(&());
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Message, vec!("Hi".to_string(), "Hello".to_string()))))
}

//result

#[test]
fn result_completed() {
    let mut parsed = parse("n -> msg seq Hi Hello");
    parsed.advance(&());
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Completed, single_data(RuleResultKind::Message, vec!("Hello".to_string()))))
}

#[test]
fn result_hit() {
    let mut parsed = parse("n -> msg seq Hi Hello");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data),
               (AdvancingResultType::Hit, single_data(RuleResultKind::Message, vec!("Hi".to_string()))))
}

#[test]
fn multiple_result_completed() {
    let mut parsed = parse("n -> msg Hi Hello; reward seq pkt badge");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data), (AdvancingResultType::Completed, Some(
        vec!(
            (RuleResultKind::Message, vec!("Hi".to_string(), "Hello".to_string())),
            (RuleResultKind::Reward, vec!("pkt".to_string(), "badge".to_string()))
        )
    )))
}

#[test]
fn multiple_result_hit() {
    let mut parsed = parse("n -> msg Hi Hello; reward seq pkt badge emote cookie");
    let adv = parsed.advance(&());
    assert_eq!((adv.res_type, adv.data), (AdvancingResultType::Hit, Some(
        vec!(
            (RuleResultKind::Message, vec!("Hi".to_string(), "Hello".to_string())),
            (RuleResultKind::Reward, vec!("pkt".to_string(), "badge".to_string()))
        )
    )))
}