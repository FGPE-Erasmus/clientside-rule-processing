use chrono::{Days, NaiveDate};
use crate::compound_rule::{CompoundRule, CompoundRuleSeq};
use crate::{AdvancingResult, AdvancingResultType};

mod rule;

pub(super) fn rule_advance(rule: &mut CompoundRule, data: &(&String, NaiveDate)) -> AdvancingResult<()> {
    let adv_res = match rule.seq {
        CompoundRuleSeq::Any => rule_any_advance(&mut rule.values, &data.0),
        CompoundRuleSeq::All => rule_all_advance(&mut rule.values, &data.0),
        CompoundRuleSeq::Order => rule_order_advance(&mut rule.values, &data.0),
        CompoundRuleSeq::Streak(s_data, ref mut s_values) =>
            rule_streak_advance(s_values, s_data, &data.1)
    };
    match adv_res.res_type {
        AdvancingResultType::None => AdvancingResult::empty(),
        AdvancingResultType::Hit => {
            rule::clean(rule, adv_res.data
                .expect("CompoundRule's hit guarantees index presence"));
            AdvancingResult::hit(None)
        }
        AdvancingResultType::Restarted => {
            // restarted type should only occur in Advancing::advance
            unreachable!()
        }
        AdvancingResultType::Completed => {
            rule.every -= 1;
            if rule.every == 0 {
                rule.iterations -= 1;
                AdvancingResult::completed(None)
            } else {
                rule.seq = rule.og_seq.clone();
                rule.values = rule.og_values.clone();
                AdvancingResult::hit(None)
            }
        }
    }
}

fn rule_streak_advance(values: &mut Vec<NaiveDate>, streak_data: u32, data: &NaiveDate)
    -> AdvancingResult<usize> {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let first = values.first().unwrap();
        if first.eq(&NaiveDate::default()) {
            if values.len() == 1 {
                AdvancingResult::completed(Some(0))
            } else {
                values.remove(1);
                values.insert(1, data.checked_add_days(Days::new(1))
                    .expect("real date won't suffer from adding a day"));
                AdvancingResult::hit(Some(0))
            }
        } else {
            if first.eq(data) {
                if values.len() == 1 {
                    AdvancingResult::completed(Some(0))
                } else {
                    values.remove(1);
                    values.insert(1, data.checked_add_days(Days::new(1))
                        .expect("real date won't suffer from adding a day"));
                    AdvancingResult::hit(Some(0))
                }
            } else {
                values.clear();
                for _ in 0..streak_data {
                    values.push(NaiveDate::default());
                }
                values.remove(1);
                values.insert(1, data.checked_add_days(Days::new(1))
                    .expect("real date won't suffer from adding a day"));
                AdvancingResult::hit(Some(0))
            }
        }
    }
}

fn rule_order_advance(values: &mut Vec<String>, data: &String) -> AdvancingResult<usize> {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let first = values
            .first()
            .unwrap();
        if first.eq(data) {
            if values.len() == 1 {
                AdvancingResult::completed(Some(0))
            } else {
                AdvancingResult::hit(Some(0))
            }
        } else {
            AdvancingResult::empty()
        }
    }
}

fn rule_all_advance(values: &mut Vec<String>, data: &String) -> AdvancingResult<usize> {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let found = values
            .iter()
            .position(|v| v.eq(data));
        if found.is_some() {
            if values.len() == 1 {
                AdvancingResult::completed(found)
            } else {
                AdvancingResult::hit(found)
            }
        } else {
            AdvancingResult::empty()
        }
    }
}

fn rule_any_advance(values: &mut Vec<String>, data: &String) -> AdvancingResult<usize> {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let found = values
            .iter()
            .any(|v| v.eq(data));
        if found {
            AdvancingResult::completed(None)
        } else {
            AdvancingResult::empty()
        }
    }
}

pub(super) fn rule_reset(rule: &mut CompoundRule) {
    rule.every = rule.og_every;
    rule.seq = rule.og_seq.clone();
    rule.values = rule.og_values.clone();
}

pub(super) fn rule_needs_reset(iterations: i32) -> bool {
    iterations != 0
}