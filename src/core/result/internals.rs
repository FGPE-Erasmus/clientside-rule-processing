use std::ops::Range;
use rand::{Rng, thread_rng};
use crate::core::result::Selector;

pub(super) fn process(amount: usize, selector: &Selector, vals: &mut Vec<String>) -> (Vec<String>, bool) {
    match selector {
        Selector::All => process_all(vals),
        Selector::Seq => process_seq(amount, vals),
        Selector::Random => process_random(amount, vals),
        Selector::RandomOnce => process_random_once(amount, vals),
        Selector::Choice => process_choice(vals)
    }
}

fn process_all(vals: &mut Vec<String>) -> (Vec<String>, bool) {
    (vals
        .drain(..)
        .collect(),
     true)
}

fn process_seq(amount: usize, vals: &mut Vec<String>) -> (Vec<String>, bool) {
    (vals
        .drain(0..amount)
        .collect(),
     vals.is_empty()
    )
}

fn process_random(amount: usize, vals: &mut Vec<String>) -> (Vec<String>, bool) {
    let range = random_base(amount, vals.len());
    (vals[range].iter().cloned().collect(), true)
}

fn process_random_once(amount: usize, vals: &mut Vec<String>) -> (Vec<String>, bool) {
    (vals
        .drain(random_base(amount, vals.len()))
        .collect(),
     vals.is_empty())
}

fn random_base(amount: usize, vals_size: usize) -> Range<usize> {
    let pairs = vals_size / amount;
    let chosen_pair = thread_rng().gen_range(1..=pairs);
    let real_i = (chosen_pair * amount) - amount;
    real_i..real_i + amount
}

fn process_choice(vals: &mut Vec<String>) -> (Vec<String>, bool) {
    (Clone::clone(vals), false)
}