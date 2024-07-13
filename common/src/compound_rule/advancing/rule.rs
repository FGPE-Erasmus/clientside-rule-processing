use crate::compound_rule::{CompoundRule, CompoundRuleSeq};

pub(super) fn clean(rule: &mut CompoundRule, index: usize) {
    if let CompoundRuleSeq::Streak(_, ref mut values) = rule.seq {
        values.remove(index);
    } else {
        rule.values.remove(index);
    }
}