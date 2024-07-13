use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime};
use crate::{Advancing, AdvancingResult, AdvancingResultType};
use crate::event::EventPartValue;
use crate::simple_rule::{SimpleRule, SimpleRulePart, WrappedSimpleRulePart};

pub(super) fn clean_parts(rule: &mut SimpleRule, parts_res: &Vec<(String, AdvancingResult<usize>)>) {
    parts_res
        .iter()
        .for_each(|(k, v)| {
            if let Some(index) = v.data {
                if let Some(item) = rule.parts.get_mut(k) {
                    match item {
                        WrappedSimpleRulePart::Number(ref mut p) => {
                            p.values.remove(index);
                        }
                        WrappedSimpleRulePart::Time(ref mut p) => {
                            p.values.remove(index);
                        }
                        WrappedSimpleRulePart::Date(ref mut p) => {
                            p.values.remove(index);
                        }
                    };
                }
            }
        });
}

pub(super) fn all_parts_completed(parts_res: &Vec<(String, AdvancingResult<usize>)>) -> bool {
    parts_res
        .iter()
        .all(|(_, val)| val.res_type == AdvancingResultType::Completed)
}

pub(super) fn advance_part(rule_parts: &mut HashMap<String, WrappedSimpleRulePart>,
                           part_name: &String, event_pv: &EventPartValue)
                           -> Option<(String, AdvancingResult<usize>)> {
    if let Some(wrap_p) = rule_parts.get_mut(part_name) {
        let adv_res = match wrap_p {
            WrappedSimpleRulePart::Number(ref mut p) =>
                advance_num_part(p, event_pv),
            WrappedSimpleRulePart::Time(ref mut p) =>
                advance_time_part(p, event_pv),
            WrappedSimpleRulePart::Date(ref mut p) =>
                advance_date_part(p, event_pv)
        };
        if let AdvancingResultType::None = adv_res.res_type {
            None
        } else {
            Some((part_name.clone(), adv_res))
        }
    } else {
        Some((part_name.clone(), AdvancingResult::completed(None)))
    }
}

fn advance_date_part(part: &mut SimpleRulePart<NaiveDate>, event_pv: &EventPartValue) -> AdvancingResult<usize> {
    if let EventPartValue::DateBased(e_val) = event_pv {
        part.advance(e_val)
    } else {
        unreachable!("logical contract guarantees proper flow")
    }
}

fn advance_time_part(part: &mut SimpleRulePart<NaiveTime>, event_pv: &EventPartValue) -> AdvancingResult<usize> {
    if let EventPartValue::TimeBased(e_val) = event_pv {
        part.advance(e_val)
    } else {
        unreachable!("logical contract guarantees proper flow")
    }
}

fn advance_num_part(part: &mut SimpleRulePart<u32>, event_pv: &EventPartValue) -> AdvancingResult<usize> {
    if let EventPartValue::NumberBased(e_val) = event_pv {
        part.advance(e_val)
    } else {
        unreachable!("logical contract guarantees proper flow")
    }
}