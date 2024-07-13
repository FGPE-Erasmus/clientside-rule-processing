use common::compound_rule::NamedCompoundRule;
use common::rule_result::NamedRuleResult;
use common::simple_rule::NamedSimpleRule;
use crate::parsing::compound_rule::error::CompoundRuleParseError;
use crate::parsing::rule_result::error::RuleResultParseError;
use crate::parsing::simple_rule::error::SimpleRuleParseError;

pub mod simple_rule;
pub mod compound_rule;
pub mod rule_result;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub(super) struct Parser;

impl Parser {
    pub(super) fn parse_simple_rule(input: &str) -> Result<NamedSimpleRule, SimpleRuleParseError> {
        let data = <Self as pest::Parser<_>>::parse(Rule::simple_rule, input)
            .map_err(|_| SimpleRuleParseError::NoMatch)?
            .next()
            .expect("match guarantees non-empty iterator");
        simple_rule::parse(data)
    }
    pub(super) fn parse_compound_rule(input: &str) -> Result<NamedCompoundRule, CompoundRuleParseError> {
        let data = <Self as pest::Parser<_>>::parse(Rule::compound_rule, input)
            .map_err(|_| CompoundRuleParseError::NoMatch)?
            .next()
            .expect("match guarantees non-empty iterator");
        compound_rule::parse(data)
    }
    pub(super) fn parse_rule_result(input: &str) -> Result<NamedRuleResult, RuleResultParseError> {
        let data = <Self as pest::Parser<_>>::parse(Rule::rule_result, input)
            .map_err(|_| RuleResultParseError::NoMatch)?
            .next()
            .expect("match guarantees non-empty iterator");
        rule_result::parse(data)
    }
}