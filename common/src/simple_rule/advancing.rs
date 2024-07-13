use chrono::{Days, NaiveDate};
use chrono::Datelike;

use crate::{Advancing, AdvancingResult};
use crate::event::Event;
use crate::simple_rule::{SimpleRule, SimpleRulePart, SimpleRulePartValue, SimpleRuleSeq, WrappedSimpleRulePart};

mod rule;

pub(super) fn rule_advance(rule: &mut SimpleRule, data: &Event) -> AdvancingResult<()> {
    let parts_adv_res: Vec<_> = data.parts
        .iter()
        .map_while(|(k, v)| rule::advance_part(&mut rule.parts, k, v))
        .collect();
    if parts_adv_res.len() == data.parts.len() {
        if rule::all_parts_completed(&parts_adv_res) {
            rule.iterations -= 1;
            AdvancingResult::completed(None)
        } else {
            rule::clean_parts(rule, &parts_adv_res);
            AdvancingResult::hit(None)
        }
    } else {
        AdvancingResult::empty()
    }
}

pub(super) fn rule_reset(rule: &mut SimpleRule) {
    rule.parts
        .values_mut()
        .for_each(|v| match v {
            WrappedSimpleRulePart::Number(ref mut p) =>
                p.reset(),
            WrappedSimpleRulePart::Time(ref mut p) =>
                p.reset(),
            WrappedSimpleRulePart::Date(ref mut p) =>
                p.reset()
        });
}

pub(super) fn part_date_advance(part: &mut SimpleRulePart<NaiveDate>, data: &NaiveDate) -> AdvancingResult<usize> {
    match part.seq {
        SimpleRuleSeq::Any => part_universal_any_advance(&mut part.values, data),
        SimpleRuleSeq::All => part_universal_all_advance(&mut part.values, data),
        SimpleRuleSeq::Order => part_universal_order_advance(&mut part.values, data),
        SimpleRuleSeq::Streak(streak_val) => part_date_streak_advance(&mut part.values, data, streak_val),
        SimpleRuleSeq::Selected(day_val) => part_date_selected_advance(data, day_val)
    }
}

fn part_date_selected_advance(data: &NaiveDate, day_val: u8) -> AdvancingResult<usize> {
    if day_val == 0 {
        AdvancingResult::completed(None)
    } else {
        let nr_from_mon = data.weekday().number_from_monday();
        if nr_from_mon as i32 - day_val as i32 == 0 {
            AdvancingResult::completed(None)
        } else {
            AdvancingResult::empty()
        }
    }
}

fn part_date_streak_advance(values: &mut Vec<SimpleRulePartValue<NaiveDate>>,
                            data: &NaiveDate, streak_val: u32) -> AdvancingResult<usize> {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let first = values.first().unwrap();
        if first.matches(&NaiveDate::default()) {
            if values.len() == 1 {
                AdvancingResult::completed(Some(0))
            } else {
                values.remove(1);
                values.insert(1, SimpleRulePartValue::exact(
                    data.checked_add_days(Days::new(1))
                        .expect("real date won't suffer from adding a day")
                ));
                AdvancingResult::hit(Some(0))
            }
        } else {
            if first.matches(data) {
                if values.len() == 1 {
                    AdvancingResult::completed(Some(0))
                } else {
                    values.remove(1);
                    values.insert(1, SimpleRulePartValue::exact(
                        data.checked_add_days(Days::new(1))
                            .expect("real date won't suffer from adding a day")
                    ));
                    AdvancingResult::hit(Some(0))
                }
            } else {
                values.clear();
                for _ in 0..streak_val {
                    values.push(SimpleRulePartValue::exact(NaiveDate::default()));
                }
                values.remove(1);
                values.insert(1, SimpleRulePartValue::exact(
                    data.checked_add_days(Days::new(1))
                        .expect("real date won't suffer from adding a day")
                ));
                AdvancingResult::hit(Some(0))
            }
        }
    }
}

pub(super) fn part_universal_advance<T>(part: &mut SimpleRulePart<T>, data: &T) -> AdvancingResult<usize>
where T: PartialEq + Eq + PartialOrd + Ord + Clone {
    match part.seq {
        SimpleRuleSeq::Any => part_universal_any_advance(&mut part.values, data),
        SimpleRuleSeq::All => part_universal_all_advance(&mut part.values, data),
        SimpleRuleSeq::Order => part_universal_order_advance(&mut part.values, data),
        _ => panic!("unsupported seq for this SimpleRulePart")
    }
}

fn part_universal_order_advance<T>(values: &mut Vec<SimpleRulePartValue<T>>, data: &T) -> AdvancingResult<usize>
where T: PartialEq + Eq + PartialOrd + Ord + Clone {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let first = values.first().unwrap();
        if first.matches(data) {
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

fn part_universal_all_advance<T>(values: &mut Vec<SimpleRulePartValue<T>>, data: &T) -> AdvancingResult<usize>
where T: PartialEq + Eq + PartialOrd + Ord + Clone {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let found = values
            .iter()
            .position(|v| v.matches(data));
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

fn part_universal_any_advance<T>(values: &mut Vec<SimpleRulePartValue<T>>, data: &T) -> AdvancingResult<usize>
where T: PartialEq + Eq + PartialOrd + Ord + Clone {
    if values.is_empty() {
        AdvancingResult::completed(None)
    } else {
        let found = values
            .iter()
            .position(|v| v.matches(data));
        if found.is_some() {
            AdvancingResult::completed(None)
        } else {
            AdvancingResult::empty()
        }
    }
}

pub(super) fn part_reset<T>(part: &mut SimpleRulePart<T>) where T: Clone {
    part.values = part.og_values.clone();
}

pub(super) fn part_needs_reset() -> bool {
    false
}

pub(super) fn rule_needs_reset(iterations: i32) -> bool {
    iterations != 0
}