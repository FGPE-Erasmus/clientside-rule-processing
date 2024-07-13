use std::ops::Range;
use rand::{Rng, thread_rng};
use crate::{Advancing, AdvancingResult, AdvancingResultType};
use crate::rule_result::{RuleResultKind, RuleResultSeq, RuleResultValue};

pub(super) fn result_advance(values: &mut Vec<RuleResultValue>) -> AdvancingResult<Vec<(RuleResultKind, Vec<String>)>> {
    let adv_res: Vec<_> = values
        .iter_mut()
        .map(|v| v.advance(&()))
        .collect();
    let all_completed = adv_res
        .iter()
        .all(|v| v.res_type == AdvancingResultType::Completed);
    let res: Vec<_> = adv_res
        .into_iter()
        .filter_map(|v| v.data)
        .collect();
    if all_completed {
        AdvancingResult::completed(Some(res))
    } else {
        AdvancingResult::hit(Some(res))
    }
}

pub(super) fn result_reset() {
}

pub(super) fn result_needs_reset() -> bool {
    false
}

pub(super) fn value_advance(value: &mut RuleResultValue) -> AdvancingResult<(RuleResultKind, Vec<String>)> {
    let arg_amount = decide_vals_amount(&value.kind);
    let args = match value.seq {
        RuleResultSeq::All => value_all_advance(&mut value.values),
        RuleResultSeq::Order => value_order_advance(&mut value.values, arg_amount),
        RuleResultSeq::Random => value_random_advance(&mut value.values, arg_amount),
        RuleResultSeq::RandomOnce => value_random_once_advance(&mut value.values, arg_amount),
        RuleResultSeq::Choice => value_choice_advance(&mut value.values)
    };
    let data = Some((value.kind.clone(), args));
    let completed = match value.seq {
        RuleResultSeq::All | RuleResultSeq::Random | RuleResultSeq::Choice => true,
        RuleResultSeq::Order | RuleResultSeq::RandomOnce => value.values.is_empty()
    };
    if completed {
        AdvancingResult::completed(data)
    } else {
        AdvancingResult::hit(data)
    }
}

fn value_choice_advance(values: &mut Vec<String>) -> Vec<String> {
    values.clone()
}

fn value_random_once_advance(values: &mut Vec<String>, arg_amount: usize) -> Vec<String> {
    values
        .drain(random_args_range(arg_amount, values.len()))
        .collect()
}

fn value_random_advance(values: &mut Vec<String>, arg_amount: usize) -> Vec<String> {
    values[random_args_range(arg_amount, values.len())]
        .iter()
        .cloned()
        .collect()
}

fn random_args_range(arg_amount: usize, vals_size: usize) -> Range<usize> {
    let chosen_pair = thread_rng()
        .gen_range(1..=vals_size / arg_amount);
    let real_i = (chosen_pair * arg_amount) - arg_amount;
    real_i..real_i + arg_amount
}

fn value_order_advance(values: &mut Vec<String>, arg_amount: usize) -> Vec<String> {
    values
        .drain(0..arg_amount)
        .collect()
}

fn value_all_advance(values: &mut Vec<String>) -> Vec<String> {
    values
        .drain(..)
        .collect()
}

fn decide_vals_amount(kind: &RuleResultKind) -> usize {
    match kind {
        RuleResultKind::Offer => 3,
        RuleResultKind::Reward => 2,
        _ => 1
    }
}

pub(super) fn value_reset(value: &mut RuleResultValue) {
    value.values = value.og_values.clone();
}

pub(super) fn value_needs_reset(iterations: i32) -> bool {
    iterations == -1
}