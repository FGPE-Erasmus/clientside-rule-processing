use chrono::{Days, NaiveDate};
use crate::core::complex_rule::element::{Element, Seq};

pub(super) fn fired(element: &mut Element, data: &String, date_data: &NaiveDate) -> Option<usize> {
    match element.seq {
        Seq::Any => any_fired(&element.vals, data),
        Seq::All => all_fired(&element.vals, data),
        Seq::Order => order_fired(&element.vals, data),
        Seq::Streak(streak) => streak_fired(element.vals.first().as_ref().unwrap(),
                                            element.streak_data.as_mut().unwrap(),
                                            data, &date_data, streak)
    }
}

pub(super) fn completed(element: &mut Element, date_data: &NaiveDate, data_pos: &Option<usize>) -> bool {
    match element.seq {
        Seq::Any => true,
        Seq::All => all_completed(&mut element.vals, data_pos),
        Seq::Order => order_completed(&mut element.vals, data_pos),
        Seq::Streak(_) => streak_completed(element.streak_data.as_mut().unwrap(), date_data)
    }
}

fn any_fired(vals: &Vec<String>, data: &String) -> Option<usize> {
    if vals.is_empty() {
        Some(0)
    } else {
        vals
            .iter()
            .position(|v| v.eq(data))
    }
}

fn all_fired(vals: &Vec<String>, data: &String) -> Option<usize> {
    if vals.is_empty() {
        Some(0)
    } else {
        vals
            .iter()
            .position(|v| v.eq(data))
    }
}

fn all_completed(vals: &mut Vec<String>, data_pos: &Option<usize>) -> bool {
    if vals.is_empty() {
        true
    } else {
        vals.remove(data_pos.unwrap());
        vals.is_empty()
    }
}

fn order_completed(vals: &mut Vec<String>, data_pos: &Option<usize>) -> bool {
    if vals.is_empty() {
        true
    } else {
        vals.remove(data_pos.unwrap());
        vals.is_empty()
    }
}

fn streak_completed(vals: &mut Vec<NaiveDate>, data: &NaiveDate) -> bool {
    if vals.is_empty() {
        true
    } else {
        if vals.len() == 1 {
            vals.clear();
            true
        } else {
            let first = vals.remove(0);
            let next = if first.eq(&NaiveDate::default()) {
                data.checked_add_days(Days::new(1)).unwrap()
            } else {
                first.checked_add_days(Days::new(1)).unwrap()
            };
            vals.remove(0);
            vals.insert(0, next);
            false
        }
    }
}

fn order_fired(vals: &Vec<String>, data: &String) -> Option<usize> {
    if vals.is_empty() {
        Some(0)
    } else {
        if vals.first().unwrap().eq(data) {
            Some(0)
        } else {
            None
        }
    }
}

fn streak_fired(val: &String, date_vals: &mut Vec<NaiveDate>, data: &String,
                date_data: &NaiveDate, streak: usize) -> Option<usize> {
    if date_vals.is_empty() {
        Some(0)
    } else {
        if !val.eq(data) {
            return None
        }
        let first = date_vals.first().unwrap();
        if first.eq(&NaiveDate::default()) {
            Some(0)
        } else {
            if !first.eq(date_data) {
                date_vals.clear();
                for _ in 0..streak {
                    date_vals.push(NaiveDate::default());
                }
            }
            Some(0)
        }
    }
}