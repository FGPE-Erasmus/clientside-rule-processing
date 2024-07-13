use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub enum CompoundRuleParseError {
    NoMatch, UnsupportedSeq, IncorrectContent
}

impl Debug for CompoundRuleParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for CompoundRuleParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CompoundRuleParseError::NoMatch => "couldn't match input with the rule",
            CompoundRuleParseError::UnsupportedSeq => "provided unsupported seq qualifier",
            CompoundRuleParseError::IncorrectContent => "one of the content value is incorrect"
        };
        f.write_str(msg)
    }
}

impl Error for CompoundRuleParseError {}