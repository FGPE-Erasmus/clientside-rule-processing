use common::rule_result::{NamedRuleResult, RuleResult, RuleResultKind, RuleResultSeq, RuleResultValue};
use pest::iterators::Pair;
use crate::parsing::Rule;
use crate::parsing::rule_result::error::RuleResultParseError;

pub mod error;

pub(super) fn parse(data: Pair<Rule>) -> Result<NamedRuleResult, RuleResultParseError> {
    let mut name = String::new();
    let mut values = Vec::new();
    for data_chunk in data.into_inner() {
        match data_chunk.as_rule() {
            Rule::name => name.push_str(data_chunk.as_str()),
            Rule::result_part => {
                let mut iterations = 1;
                let mut kind = RuleResultKind::Message;
                let mut seq = RuleResultSeq::All;
                let mut args = Vec::new();
                for data_chunk in data_chunk.into_inner() {
                    match data_chunk.as_rule() {
                        Rule::repeat => iterations = -1,
                        Rule::result_kind => kind = parse_kind(data_chunk.as_str())?,
                        Rule::result_seq => seq = parse_seq(data_chunk.as_str())?,
                        Rule::result_arg => args.push(data_chunk.as_str().to_owned()),
                        _ => unreachable!()
                    }
                }
                values.push(RuleResultValue::new(iterations, kind, seq, args));
            }
            _ => unreachable!()
        }
    }
    Ok(NamedRuleResult::new(name, RuleResult::new(values)))
}

fn parse_seq(input: &str) -> Result<RuleResultSeq, RuleResultParseError> {
    let res = match input {
        "all" => RuleResultSeq::All,
        "seq" => RuleResultSeq::Order,
        "random" => RuleResultSeq::Random,
        "random_once" => RuleResultSeq::RandomOnce,
        "choice" => RuleResultSeq::Choice,
        _ => return Err(RuleResultParseError::UnsupportedSeq)
    };
    Ok(res)
}

fn parse_kind(input: &str) -> Result<RuleResultKind, RuleResultParseError> {
    let res = match input {
        "msg" => RuleResultKind::Message,
        "reward" => RuleResultKind::Reward,
        "offer" => RuleResultKind::Offer,
        "open" => RuleResultKind::Open,
        "restart" => RuleResultKind::Restart,
        _ => return Err(RuleResultParseError::UnsupportedKind)
    };
    Ok(res)
}