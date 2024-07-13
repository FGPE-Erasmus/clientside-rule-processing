use std::error::Error;
use common::compound_rule::NamedCompoundRule;
use common::rule_result::NamedRuleResult;
use common::simple_rule::NamedSimpleRule;

pub(super) fn parse_simple_rules(input: &str) -> anyhow::Result<Vec<NamedSimpleRule>> {
    parse_data(input, |s| parser::parse_simple_rule(s))
}

pub(super) fn parse_compound_rules(input: &str) -> anyhow::Result<Vec<NamedCompoundRule>> {
    parse_data(input, |s| parser::parse_compound_rule(s))
}

pub(super) fn parse_rule_results(input: &str) -> anyhow::Result<Vec<NamedRuleResult>> {
    parse_data(input, |s| parser::parse_rule_result(s))
}

fn parse_data<T, E>(input: &str, parsing_fn: impl Fn(&str) -> Result<T, E>) -> anyhow::Result<Vec<T>>
where E: Error {
    let parsed = input
        .lines()
        .filter_map(|s| {
            let p = parsing_fn(s);
            if let Err(err) = &p {
                tracing::error!("could not parse line - details: {err}")
            }
            p.ok()
        })
        .collect();
    Ok(parsed)
}