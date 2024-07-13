use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub enum RuleResultParseError {
    NoMatch, UnsupportedSeq, UnsupportedKind
}

impl Debug for RuleResultParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for RuleResultParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            RuleResultParseError::NoMatch => "couldn't match input with the result",
            RuleResultParseError::UnsupportedSeq => "provided unsupported seq qualifier",
            RuleResultParseError::UnsupportedKind => "provided unsupported kind qualifier",
        };
        f.write_str(msg)
    }
}

impl Error for RuleResultParseError {}