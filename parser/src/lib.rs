use common::compound_rule::NamedCompoundRule;
use common::rule_result::NamedRuleResult;
use common::simple_rule::NamedSimpleRule;
use crate::parsing::compound_rule::error::CompoundRuleParseError;
use crate::parsing::rule_result::error::RuleResultParseError;
use crate::parsing::simple_rule::error::SimpleRuleParseError;

pub mod parsing;

pub fn parse_simple_rule(input: &str) -> Result<NamedSimpleRule, SimpleRuleParseError> {
    parsing::Parser::parse_simple_rule(input)
}

pub fn parse_compound_rule(input: &str) -> Result<NamedCompoundRule, CompoundRuleParseError> {
    parsing::Parser::parse_compound_rule(input)
}

pub fn parse_rule_result(input: &str) -> Result<NamedRuleResult, RuleResultParseError> {
    parsing::Parser::parse_rule_result(input)
}