use chrono::NaiveDate;
use common::compound_rule::{CompoundRule, CompoundRuleSeq, NamedCompoundRule};
use pest::iterators::Pair;

use crate::parsing::compound_rule::error::CompoundRuleParseError;
use crate::parsing::compound_rule::error::CompoundRuleParseError::IncorrectContent;
use crate::parsing::Rule;

pub mod error;

pub(super) fn parse(data: Pair<Rule>) -> Result<NamedCompoundRule, CompoundRuleParseError> {
    let mut name = String::new();
    let mut iterations = 1;
    let mut every = 1;
    let mut seq = CompoundRuleSeq::Any;
    let mut content = Vec::new();
    for data_chunk in data.into_inner() {
        match data_chunk.as_rule() {
            Rule::name => name.push_str(data_chunk.as_str()),
            Rule::repeat => iterations = -1,
            Rule::compound_rule_every_content => every = {
                let parsed = parse_numeric_val(data_chunk.as_str())?;
                if parsed > 0 {
                    parsed
                } else {
                    return Err(IncorrectContent)
                }
            },
            Rule::compound_rule_seq => seq = parse_seq(data_chunk.as_str())?,
            Rule::compound_rule_content => content.push(data_chunk.as_str().to_owned()),
            _ => unreachable!()
        }
    }
    Ok(NamedCompoundRule::new(name, CompoundRule::new(iterations, every, seq, content)))
}

fn parse_seq(input: &str) -> Result<CompoundRuleSeq, CompoundRuleParseError> {
    let mut chunks = input.split(' ');
    let res = match chunks.next().unwrap_or("any") {
        "any" => CompoundRuleSeq::Any,
        "all" => CompoundRuleSeq::All,
        "seq" => CompoundRuleSeq::Order,
        "streak" => {
            let streak_num = if let Some(item) = chunks.next() {
                let parsed = parse_numeric_val(item)?;
                if parsed > 0 {
                    parsed
                } else {
                    return Err(IncorrectContent);
                }
            } else {
                return Err(IncorrectContent);
            };
            let mut streak_vals = Vec::new();
            for _ in 0..streak_num {
                streak_vals.push(NaiveDate::default());
            }
            CompoundRuleSeq::Streak(streak_num, streak_vals)
        },
        _ => return Err(CompoundRuleParseError::UnsupportedSeq)
    };
    Ok(res)
}

fn parse_numeric_val(input: &str) -> Result<u32, CompoundRuleParseError> {
    Ok(input.parse().map_err(|_| IncorrectContent)?)
}