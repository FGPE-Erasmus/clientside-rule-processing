use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub enum SimpleRuleParseError {
    NoMatch, UnsupportedSeq, IncorrectContent
}

impl Debug for SimpleRuleParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for SimpleRuleParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            SimpleRuleParseError::NoMatch => "couldn't match input with the rule",
            SimpleRuleParseError::UnsupportedSeq => "provided unsupported seq qualifier",
            SimpleRuleParseError::IncorrectContent => "one of the content value is incorrect"
        };
        f.write_str(msg)
    }
}

impl Error for SimpleRuleParseError {}
