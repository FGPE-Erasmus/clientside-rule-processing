use std::collections::HashMap;

use chrono::NaiveDate;
use common::{Advancing, AdvancingResultType};
use common::compound_rule::CompoundRule;
use common::event::{Event, EventPartValue};
use common::rule_result::{RuleResult, RuleResultKind};
use common::simple_rule::SimpleRule;
use crate::core::State;

pub(super) fn process_rule_results(state: &mut State,
                                   c_rules: Vec<String>) -> Vec<(RuleResultKind, Vec<String>)> {
    let results = advance_clean_results(&mut state.enabled_rule_results, &mut state.disabled_rule_results, c_rules);
    apply_special_actions(state, &results);
    results
}

fn apply_special_actions(state: &mut State, results: &Vec<(RuleResultKind, Vec<String>)>) {
    for (kind, args) in results {
        if let RuleResultKind::Restart = kind {
            args.iter()
                .for_each(|arg| {
                    if let Some(rule) = state.disabled_simple_rules.remove_entry(arg) {
                        state.enabled_simple_rules.insert(rule.0, rule.1);
                    }
                    if let Some(rule) = state.disabled_compound_rules.remove_entry(arg) {
                        state.enabled_compound_rules.insert(rule.0, rule.1);
                    }
                })
        }
    }
}

fn advance_clean_results(e_results: &mut HashMap<String, RuleResult>,
                         d_results: &mut HashMap<String, RuleResult>,
                         c_rules: Vec<String>) -> Vec<(RuleResultKind, Vec<String>)> {
    c_rules
        .into_iter()
        .filter_map(|name| {
            if let Some(item) = e_results.get_mut(&name) {
                let adv_res = item.advance(&());
                if let AdvancingResultType::Completed = adv_res.res_type {
                    let entry = e_results.remove_entry(&name).unwrap();
                    d_results.insert(entry.0, entry.1);
                }
                Some(adv_res.data.unwrap())
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub(super) fn process_compound_rules(e_rules: &mut HashMap<String, CompoundRule>,
                                     d_rules: &mut HashMap<String, CompoundRule>,
                                     c_simple_rules: &Vec<String>,
                                     event: &Event) -> Vec<String> {
    let adv_res = advance_compound_rules(e_rules, c_simple_rules, get_event_date(event));
    clean_compound_rules(adv_res, e_rules, d_rules)
}

fn get_event_date(event: &Event) -> NaiveDate {
    if let EventPartValue::DateBased(date) = event.parts.get("on")
        .expect("date presence is guaranteed in case we're dealing with Compound/Simple rules") {
        *date
    } else {
        unreachable!("date must be stored in DateBased EventPartValue")
    }
}

fn clean_compound_rules(completed: Vec<(AdvancingResultType, String)>,
                        e_rules: &mut HashMap<String, CompoundRule>,
                        d_rules: &mut HashMap<String, CompoundRule>) -> Vec<String> {
    completed
        .into_iter()
        .map(|(res_type, name)| {
            if let AdvancingResultType::Completed = res_type {
                let entry = e_rules.remove_entry(&name).unwrap();
                d_rules.insert(entry.0, entry.1);
            }
            name
        })
        .collect()
}

fn advance_compound_rules(e_rules: &mut HashMap<String, CompoundRule>,
                          c_simple_rules: &Vec<String>,
                          date: NaiveDate) -> Vec<(AdvancingResultType, String)> {
    c_simple_rules
        .iter()
        .map(|v|
            e_rules
                .iter_mut()
                .filter_map(|(n, r)| {
                    let adv_res = r.advance(&(v, date));
                    match adv_res.res_type {
                        AdvancingResultType::Restarted | AdvancingResultType::Completed => Some((adv_res.res_type, n.clone())),
                        _ => None
                    }
                })
                .collect::<Vec<_>>()
        )
        .flatten()
        .collect()
}

pub(super) fn process_simple_rules(e_rules: &mut HashMap<String, SimpleRule>,
                                   d_rules: &mut HashMap<String, SimpleRule>,
                                   event: &Event) -> Vec<String> {
    let adv_res = advance_simple_rules(e_rules, event);
    clean_simple_rules(adv_res, e_rules, d_rules)
}

fn clean_simple_rules(completed: Vec<(AdvancingResultType, String)>,
                      e_rules: &mut HashMap<String, SimpleRule>,
                      d_rules: &mut HashMap<String, SimpleRule>) -> Vec<String> {
    completed
        .into_iter()
        .map(|(res_type, name)| {
            if let AdvancingResultType::Completed = res_type {
                let entry = e_rules.remove_entry(&name).unwrap();
                d_rules.insert(entry.0, entry.1);
            }
            name
        })
        .collect()
}

fn advance_simple_rules(e_rules: &mut HashMap<String, SimpleRule>, event: &Event) -> Vec<(AdvancingResultType, String)> {
    e_rules
        .iter_mut()
        .filter_map(|(name, rule)| {
            let adv_res = rule.advance(event);
            match adv_res.res_type {
                AdvancingResultType::Restarted | AdvancingResultType::Completed =>
                    Some((adv_res.res_type, name.clone())),
                _ => None
            }
        })
        .collect()
}