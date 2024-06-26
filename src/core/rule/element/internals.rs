use chrono::{Days, NaiveDate};
use crate::core::rule::element::{Border, Day, Element, Seq};

pub(super) fn universal_completed<T>(element: &mut Element<T>, data_pos: &Option<&usize>) -> bool {
    match element.seq {
        Seq::Any => true,
        Seq::All => universal_all_completed(&mut element.vals, data_pos),
        Seq::Order => universal_order_completed(&mut element.vals, data_pos),
        _ => panic!()
    }
}

pub(super) fn date_completed(element: &mut Element<NaiveDate>, data: &NaiveDate, data_pos: &Option<&usize>) -> bool {
    match &element.seq {
        Seq::Any => true,
        Seq::All => universal_all_completed(&mut element.vals, data_pos),
        Seq::Order => universal_order_completed(&mut element.vals, data_pos),
        Seq::Streak(_) => date_streak_completed(&mut element.vals, data),
        Seq::Selected(_) => date_selected_completed(&mut element.vals)
    }
}

pub(super) fn universal_fired<T>(element: &Element<T>, data: &T) -> Option<usize>
    where T: PartialEq + Eq + PartialOrd + Ord {
    match element.seq {
        Seq::Any => universal_any_fired(&element.vals, data),
        Seq::All => universal_all_fired(&element.vals, data),
        Seq::Order => universal_order_fired(&element.vals, data),
        _ => panic!()
    }
}

pub(super) fn date_fired(element: &mut Element<NaiveDate>, data: &NaiveDate) -> Option<usize> {
    match &element.seq {
        Seq::Any => universal_any_fired(&element.vals, data),
        Seq::All => universal_all_fired(&element.vals, data),
        Seq::Order => universal_order_fired(&element.vals, data),
        Seq::Streak(streak) => date_streak_fired(&mut element.vals, data, streak),
        Seq::Selected(day) => date_selected_fired(data, &day)
    }
}

fn universal_all_completed<T>(vals: &mut Vec<(Border, T, Option<T>)>, data_pos: &Option<&usize>) -> bool {
    if vals.is_empty() {
        true
    } else {
        vals.remove(*data_pos.unwrap());
        vals.is_empty()
    }
}

fn date_selected_completed(vals: &mut Vec<(Border, NaiveDate, Option<NaiveDate>)>) -> bool {
    if vals.is_empty() {
        true
    } else {
        vals.clear();
        true
    }
}

fn date_streak_completed(vals: &mut Vec<(Border, NaiveDate, Option<NaiveDate>)>, data: &NaiveDate) -> bool {
    if vals.is_empty() {
        true
    } else {
        if vals.len() == 1 {
            vals.clear();
            true
        } else {
            let first = vals.remove(0);
            let next = if super::val_matches(&first, &NaiveDate::default()) {
                (Border::Exact, data.checked_add_days(Days::new(1)).unwrap(), None)
            } else {
                (Border::Exact, first.1.checked_add_days(Days::new(1)).unwrap(), None)
            };
            vals.remove(0);
            vals.insert(0, next);
            false
        }
    }
}

fn universal_order_completed<T>(vals: &mut Vec<(Border, T, Option<T>)>, data_pos: &Option<&usize>) -> bool {
    if vals.is_empty() {
        true
    } else {
        vals.remove(*data_pos.unwrap());
        vals.is_empty()
    }
}

fn date_selected_fired(data: &NaiveDate, day: &Day) -> Option<usize> {
    if day.date_matches(data) {
        Some(0)
    } else {
        None
    }
}

fn date_streak_fired(vals: &mut Vec<(Border, NaiveDate, Option<NaiveDate>)>, data: &NaiveDate, streak: &usize) -> Option<usize> {
    if vals.is_empty() {
        Some(0)
    } else {
        let first = vals.first().unwrap();
        if super::val_matches(first, &NaiveDate::default()) {
            Some(0)
        } else {
            if !super::val_matches(first, data) {
                vals.clear();
                for _ in 0..*streak {
                    vals.push((Border::Exact, NaiveDate::default(), None));
                }
            }
            Some(0)
        }
    }
}

fn universal_any_fired<T>(vals: &Vec<(Border, T, Option<T>)>, data: &T) -> Option<usize>
    where T: PartialEq + Eq + PartialOrd + Ord {
    if vals.is_empty() {
        Some(0)
    } else {
        vals
            .iter()
            .position(|v| super::val_matches(v, data))
    }
}

fn universal_all_fired<T>(vals: &Vec<(Border, T, Option<T>)>, data: &T) -> Option<usize>
    where T: PartialEq + Eq + PartialOrd + Ord {
    if vals.is_empty() {
        Some(0)
    } else {
        vals
            .iter()
            .position(|v| super::val_matches(v, data))
    }
}

fn universal_order_fired<T>(vals: &Vec<(Border, T, Option<T>)>, data: &T) -> Option<usize>
    where T: PartialEq + Eq + PartialOrd + Ord {
    if vals.is_empty() {
        Some(0)
    } else {
        if super::val_matches(vals.first().unwrap(), data) {
            Some(0)
        } else {
            None
        }
    }
}