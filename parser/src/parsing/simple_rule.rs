use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::{FromStr, Split};

use common::simple_rule::{NamedSimpleRule, SimpleRule, SimpleRuleBorder, SimpleRulePart, SimpleRulePartValue, SimpleRuleSeq, WrappedSimpleRulePart};
use pest::iterators::Pair;
use pest::Parser;

use crate::parsing::Rule;
use crate::parsing::simple_rule::error::SimpleRuleParseError;

pub mod error;

pub(super) fn parse(data: Pair<Rule>) -> Result<NamedSimpleRule, SimpleRuleParseError> {
    let mut name = String::new();
    let mut iterations = 1;
    let mut parts = Vec::new();
    for data_chunk in data.into_inner() {
        match data_chunk.as_rule() {
            Rule::name => name.push_str(data_chunk.as_str()),
            Rule::simple_rule_part => {
                let mut kw = None;
                let mut seq = None;
                let mut cont = None;
                for data_chunk in data_chunk.into_inner() {
                    match data_chunk.as_rule() {
                        Rule::simple_rule_keyword => kw = Some(data_chunk.as_str()),
                        Rule::simple_rule_seq => seq = Some(data_chunk.as_str()),
                        Rule::simple_rule_content => cont = Some(data_chunk.as_str()),
                        _ => unreachable!()
                    }
                }
                if let Some("repeat") = kw {
                    let cont = cont.expect("match guarantees non-empty repeat");
                    iterations = if cont.eq("+") {
                        -1
                    } else {
                        cont.parse()
                            .map_err(|_| SimpleRuleParseError::IncorrectContent)?
                    }
                } else {
                    parts.push((kw, seq, cont));
                }
            }
            _ => unreachable!()
        }
    }
    if iterations == 0 {
        return Err(SimpleRuleParseError::IncorrectContent);
    }
    Ok(NamedSimpleRule::new(name, parse_rule(iterations, parts)?))
}

fn parse_rule(iterations: i32, kw_seq_cont: Vec<(Option<&str>, Option<&str>, Option<&str>)>)
    -> Result<SimpleRule, SimpleRuleParseError> {
    let mut parts: HashMap<String, WrappedSimpleRulePart> = HashMap::new();
    for data in kw_seq_cont {
        parts.insert(
            data.0.expect("hit guarantees presence of a value").to_owned(),
            parse_wrapped_part(data)?
        );
    }
    Ok(SimpleRule::new(iterations, parts))
}

fn parse_wrapped_part(kw_seq_cont: (Option<&str>, Option<&str>, Option<&str>))
    -> Result<WrappedSimpleRulePart, SimpleRuleParseError> {
    let res = match kw_seq_cont.0.expect("hit guarantees presence of a value") {
        "on" => WrappedSimpleRulePart::Date(parse_part(kw_seq_cont.1, kw_seq_cont.2)?),
        "at" => WrappedSimpleRulePart::Time(parse_part(kw_seq_cont.1, kw_seq_cont.2)?),
        _ => WrappedSimpleRulePart::Number(parse_part(kw_seq_cont.1, kw_seq_cont.2)?)
    };
    Ok(res)
}

fn parse_part<T>(seq: Option<&str>, cont: Option<&str>)
    -> Result<SimpleRulePart<T>, SimpleRuleParseError> where T: Clone + Default + FromStr {
    let cont = cont.expect("hit guarantees presence of a value");
    let seq = parse_seq(seq, &mut cont.split(','))?;
    let values = match seq {
        SimpleRuleSeq::Any | SimpleRuleSeq::All | SimpleRuleSeq::Order
            => parse_values(cont.split(','))?,
        SimpleRuleSeq::Streak(streak_val) => {
            if streak_val <= 0 {
                return Err(SimpleRuleParseError::IncorrectContent)
            }
            let mut values = Vec::new();
            for _ in 0..streak_val {
                values.push(SimpleRulePartValue::exact(T::default()));
            }
            values
        }
        SimpleRuleSeq::Selected(_) => Vec::new()
    };
    Ok(SimpleRulePart::new(seq, values))
}

fn parse_values<T>(cont: Split<char>) -> Result<Vec<SimpleRulePartValue<T>>, SimpleRuleParseError>
where T: Clone + FromStr {
    let mut vals = Vec::new();
    for c in cont {
        if !c.contains('*') {
            vals.push(parse_value(c)?)
        }
    }
    Ok(vals)
}

fn parse_value<T>(input: &str) -> Result<SimpleRulePartValue<T>, SimpleRuleParseError>
where T: Clone + FromStr {
    let mut border;
    let mut left_val;
    let mut right_val = None;
    if input.contains("..") {
        let mut content = input.split("..");
        border = SimpleRuleBorder::Between;
        left_val = content.next();
        right_val = Some(content.next()
            .ok_or_else(|| SimpleRuleParseError::IncorrectContent)?
            .replace(".", "-")
            .parse::<T>()
            .map_err(|e| SimpleRuleParseError::IncorrectContent)?);
    } else if input.contains(">=") {
        border = SimpleRuleBorder::GreaterEq;
        left_val = input.split(">=")
            .skip(1)
            .next();
    } else if input.contains("<=") {
        border = SimpleRuleBorder::LessEq;
        left_val = input.split("<=")
            .skip(1)
            .next();
    } else if input.contains(">") {
        border = SimpleRuleBorder::Greater;
        left_val = input.split(">")
            .skip(1)
            .next();
    } else if input.contains("<") {
        border = SimpleRuleBorder::Less;
        left_val = input.split("<")
            .skip(1)
            .next();
    } else {
        border = SimpleRuleBorder::Exact;
        left_val = Some(input);
    }
    Ok(SimpleRulePartValue::new(
        border,
        left_val.ok_or_else(|| SimpleRuleParseError::IncorrectContent)?
            .replace(".", "-")
            .parse::<T>()
            .map_err(|_| SimpleRuleParseError::IncorrectContent)?,
        right_val
    ))
}

fn parse_seq(seq: Option<&str>, cont: &mut Split<char>) -> Result<SimpleRuleSeq, SimpleRuleParseError> {
    let res = match seq.unwrap_or("any") {
        "any" => SimpleRuleSeq::Any,
        "all" => SimpleRuleSeq::All,
        "seq" => SimpleRuleSeq::Order,
        "streak" => SimpleRuleSeq::Streak(cont.next()
                .unwrap()
                .parse::<u32>()
                .map_err(|_| SimpleRuleParseError::IncorrectContent)?),
        "every" => SimpleRuleSeq::Selected(match cont.next().unwrap() {
            "DAY" => 0,
            "MONDAY" => 1,
            "TUESDAY" => 2,
            "WEDNESDAY" => 3,
            "THURSDAY" => 4,
            "FRIDAY" => 5,
            "SATURDAY" => 6,
            "SUNDAY" => 7,
            raw_val @ _ => {
                let val = raw_val.parse::<u8>()
                    .map_err(|_| SimpleRuleParseError::IncorrectContent)?;
                if val > 7 {
                    Err(SimpleRuleParseError::IncorrectContent)?
                } else {
                    val
                }
            }
        }),
        _ => return Err(SimpleRuleParseError::UnsupportedSeq)
    };
    Ok(res)
}