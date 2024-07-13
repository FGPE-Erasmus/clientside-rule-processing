use std::str::FromStr;
use chrono::{NaiveDate, NaiveTime};
use common::{Advancing, AdvancingResultType};
use common::event::{Event, EventPartValue};
use common::simple_rule::{SimpleRuleBorder, SimpleRulePart, SimpleRulePartValue, SimpleRuleSeq, WrappedSimpleRulePart};

fn default_event() -> Event {
    let parts = [
        ("player", EventPartValue::NumberBased(1)),
        ("did", EventPartValue::NumberBased(1)),
        ("with", EventPartValue::NumberBased(1)),
        ("in", EventPartValue::NumberBased(1)),
        ("of", EventPartValue::NumberBased(1)),
        ("on", EventPartValue::DateBased(NaiveDate::from_str("2000-01-01").unwrap())),
        ("at", EventPartValue::TimeBased(NaiveTime::from_str("10:00").unwrap())),
        ("achieving", EventPartValue::NumberBased(1)),
        ("repeat", EventPartValue::NumberBased(1))
    ].into_iter()
        .map(|(kw, val)| (kw.to_string(), val))
        .collect();
    Event::new(parts)
}

fn clear_og_vals(part: &mut WrappedSimpleRulePart) {
    match part {
        WrappedSimpleRulePart::Number(ref mut p) => p.og_values.clear(),
        WrappedSimpleRulePart::Time(ref mut p) => p.og_values.clear(),
        WrappedSimpleRulePart::Date(ref mut p) => p.og_values.clear()
    }
}

fn setup_and_advance(rule: &str, kw: &str) -> (AdvancingResultType, Option<WrappedSimpleRulePart>) {
    let mut rule = parser::parse_simple_rule(rule).unwrap().rule;
    let adv_res = rule.advance(&default_event());
    (adv_res.res_type, rule.parts.remove(kw))
}

fn assert_res(mut res: (AdvancingResultType, Option<WrappedSimpleRulePart>),
              mut expected_res: (AdvancingResultType, Option<WrappedSimpleRulePart>)) {
    if let Some(ref mut item) = res.1 {
        clear_og_vals(item)
    }
    if let Some(ref mut item) = expected_res.1 {
        clear_og_vals(item)
    }
    assert_eq!(res, expected_res)
}

//player

#[test]
fn no_player() {
    let res = setup_and_advance("n: did 1", "player");
    assert_res(res, (AdvancingResultType::Completed, None));
}

#[test]
fn asterisk_player() {
    let res = setup_and_advance("n: player *", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_asterisk_player() {
    let res = setup_and_advance("n: player any(*)", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn matching_player() {
    let res = setup_and_advance("n: player 1", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn nonmatching_player() {
    let res = setup_and_advance("n: player 2", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(2)
        ))
    );
    assert_res(res, (AdvancingResultType::None, Some(expected_part)))
}

#[test]
fn any_multiple_player() {
    let res = setup_and_advance("n: player 1,2,3", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

//action

#[test]
fn no_action() {
    let res = setup_and_advance("n: player 1", "did");
    assert_res(res, (AdvancingResultType::Completed, None))
}

#[test]
fn matching_action() {
    let res = setup_and_advance("n: did 1", "did");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn nonmatching_action() {
    let res = setup_and_advance("n: did 2", "did");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(2)
        ))
    );
    assert_res(res, (AdvancingResultType::None, Some(expected_part)))
}

//objectlike (object, location, area)

#[test]
fn no_objectlike() {
    let res = setup_and_advance("n: player 1", "with");
    assert_res(res, (AdvancingResultType::Completed, None))
}

#[test]
fn asterisk_objectlike() {
    let res = setup_and_advance("n: with *", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_asterisk_objectlike() {
    let res = setup_and_advance("n: with any(*)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn matching_objectlike() {
    let res = setup_and_advance("n: with 1", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn nonmatching_objectlike() {
    let res = setup_and_advance("n: with 2", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(2)
        ))
    );
    assert_res(res, (AdvancingResultType::None, Some(expected_part)))
}

#[test]
fn any_objectlike() {
    let res = setup_and_advance("n: with any(1)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn all_objectlike() {
    let res = setup_and_advance("n: with all(1)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn seq_objectlike() {
    let res = setup_and_advance("n: with seq(1)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Order, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn multiple_objectlike() {
    let res = setup_and_advance("n: with 1,2,3", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_multiple_objectlike() {
    let res = setup_and_advance("n: with any(1,2,3)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn all_multiple_objectlike() {
    let res = setup_and_advance("n: with all(1,2,3)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3)
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

#[test]
fn all_asterisk_objectlike() {
    let res = setup_and_advance("n: with all(*)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn seq_multiple_objectlike() {
    let res = setup_and_advance("n: with seq(1,2,3)", "with");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Order, vec!(
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3)
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

//date

#[test]
fn no_date() {
    let res = setup_and_advance("n: player 1", "on");
    assert_res(res, (AdvancingResultType::Completed, None))
}

#[test]
fn asterisk_date() {
    let res = setup_and_advance("n: on *", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_asterisk_date() {
    let res = setup_and_advance("n: on any(*)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn single_date() {
    let res = setup_and_advance("n: on 2000.01.01", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_date() {
    let res = setup_and_advance("n: on any(2000.01.01)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn all_date() {
    let res = setup_and_advance("n: on all(2000.01.01)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn multiple_date() {
    let res = setup_and_advance("n: on 2000.01.01,2000.02.02", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-02-02").unwrap()),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_multiple_date() {
    let res = setup_and_advance("n: on any(2000.01.01,2000.02.02)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-02-02").unwrap()),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn all_multiple_date() {
    let res = setup_and_advance("n: on all(2000.01.01,2000.02.02)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-02-02").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

#[test]
fn streak_one_date() {
    let res = setup_and_advance("n: on streak(1)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Streak(1), vec!(
            SimpleRulePartValue::exact(NaiveDate::default())
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn streak_three_date() {
    let res = setup_and_advance("n: on streak(3)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Streak(3), vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-02").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::default())
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

#[test]
fn failed_streak_date() {
    let mut rule = parser::parse_simple_rule("n: on streak(3)").unwrap().rule;
    rule.advance(&default_event());
    let adv_res = rule.advance(&default_event());
    let res = (adv_res.res_type, rule.parts.remove("on"));
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Streak(3), vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-02").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::default())
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

#[test]
fn every_day_date() {
    let res = setup_and_advance("n: on every(0)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Selected(0), vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn every_monday_failed_date() {
    let res = setup_and_advance("n: on every(1)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Selected(1), vec!())
    );
    assert_res(res, (AdvancingResultType::None, Some(expected_part)))
}

#[test]
fn every_saturday_date() {
    let res = setup_and_advance("n: on every(6)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Selected(6), vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn less_date() {
    let res = setup_and_advance("n: on <2010.01.01", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Less, NaiveDate::from_str("2010-01-01").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn less_eq_date() {
    let res = setup_and_advance("n: on <=2010.01.01", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::LessEq, NaiveDate::from_str("2010-01-01").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn more_date() {
    let res = setup_and_advance("n: on >1999.01.01", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, NaiveDate::from_str("1999-01-01").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn more_eq_date() {
    let res = setup_and_advance("n: on >=1999.01.01", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::GreaterEq, NaiveDate::from_str("1999-01-01").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn between_date() {
    let res = setup_and_advance("n: on 1999.01.01..2002.01.01", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Between, NaiveDate::from_str("1999-01-01").unwrap(), Some(NaiveDate::from_str("2002-01-01").unwrap()))
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn mixed_date() {
    let res = setup_and_advance("n: on all(1999.01.01..2002.01.01,>2005.01.01,2010.12.12)", "on");
    let expected_part = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, NaiveDate::from_str("2005-01-01").unwrap(), None),
            SimpleRulePartValue::new(SimpleRuleBorder::Exact, NaiveDate::from_str("2010-12-12").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

//time

#[test]
fn no_time() {
    let res = setup_and_advance("n: player 1", "at");
    assert_res(res, (AdvancingResultType::Completed, None))
}

#[test]
fn asterisk_time() {
    let res = setup_and_advance("n: at *", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_asterisk_time() {
    let res = setup_and_advance("n: at any(*)", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn single_time() {
    let res = setup_and_advance("n: at 10:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_time() {
    let res = setup_and_advance("n: at any(10:00)", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn all_time() {
    let res = setup_and_advance("n: at all(10:00)", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn multiple_time() {
    let res = setup_and_advance("n: at 10:00,12:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("12:00").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_multiple_time() {
    let res = setup_and_advance("n: at any(10:00,12:00)", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("12:00").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn all_multiple_time() {
    let res = setup_and_advance("n: at all(10:00,12:00)", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("12:00").unwrap())
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

#[test]
fn less_time() {
    let res = setup_and_advance("n: at <11:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Less, NaiveTime::from_str("11:00").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn less_eq_time() {
    let res = setup_and_advance("n: at <=11:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::LessEq, NaiveTime::from_str("11:00").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn more_time() {
    let res = setup_and_advance("n: at >9:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, NaiveTime::from_str("9:00").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn more_eq_time() {
    let res = setup_and_advance("n: at >=9:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::GreaterEq, NaiveTime::from_str("9:00").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn between_time() {
    let res = setup_and_advance("n: at 9:00..11:00", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Between, NaiveTime::from_str("9:00").unwrap(), Some(NaiveTime::from_str("11:00").unwrap()))
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn mixed_time() {
    let res = setup_and_advance("n: at all(9:00..11:00,>13:00,22:00)", "at");
    let expected_part = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, NaiveTime::from_str("13:00").unwrap(), None),
            SimpleRulePartValue::new(SimpleRuleBorder::Exact, NaiveTime::from_str("22:00").unwrap(), None)
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

//result

#[test]
fn no_result() {
    let res = setup_and_advance("n: player 1", "achieving");
    assert_res(res, (AdvancingResultType::Completed, None))
}

#[test]
fn asterisk_result() {
    let res = setup_and_advance("n: achieving *", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_asterisk_result() {
    let res = setup_and_advance("n: achieving any(*)", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!())
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn single_result() {
    let res = setup_and_advance("n: achieving 1", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_result() {
    let res = setup_and_advance("n: achieving any(1)", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn multiple_result() {
    let res = setup_and_advance("n: achieving 1,2", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn any_multiple_result() {
    let res = setup_and_advance("n: achieving any(1,2)", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn less_result() {
    let res = setup_and_advance("n: achieving <2", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Less, 2, None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn less_eq_result() {
    let res = setup_and_advance("n: achieving <=2", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::LessEq, 2, None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn more_result() {
    let res = setup_and_advance("n: achieving >0", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, 0, None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn more_eq_result() {
    let res = setup_and_advance("n: achieving >=0", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::GreaterEq, 0, None)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn between_result() {
    let res = setup_and_advance("n: achieving 0..2", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Between, 0, Some(2))
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn mixed_result() {
    let res = setup_and_advance("n: achieving all(0..2,>4,6)", "achieving");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, 4, None),
            SimpleRulePartValue::new(SimpleRuleBorder::Exact, 6, None)
        ))
    );
    assert_res(res, (AdvancingResultType::Hit, Some(expected_part)))
}

//repeat

#[test]
fn no_repeat() {
    let res = setup_and_advance("n: player 1", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn repeat_one() {
    let res = setup_and_advance("n: player 1 repeat 1", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Completed, Some(expected_part)))
}

#[test]
fn repeat_two() {
    let res = setup_and_advance("n: player 1 repeat 2", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Restarted, Some(expected_part)))
}

#[test]
fn repeat_infinite() {
    let res = setup_and_advance("n: player 1 repeat +", "player");
    let expected_part = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_res(res, (AdvancingResultType::Restarted, Some(expected_part)))
}

//rule

#[test]
fn rule_completed() {
    let mut rule = parser::parse_simple_rule("name: player 1 did 1 with 1 in 1 of 1 on 2000.01.01 at 10:00 achieving 1")
        .unwrap().rule;
    let adv_res = rule.advance(&default_event());
    assert_eq!(adv_res.res_type, AdvancingResultType::Completed)
}

#[test]
fn rule_hit() {
    let mut rule = parser::parse_simple_rule("name: player 1 did all(1,2,3) with 1 in 1 of 1 on 2000.01.01 at 10:00 achieving 1")
        .unwrap().rule;
    let adv_res = rule.advance(&default_event());
    assert_eq!(adv_res.res_type, AdvancingResultType::Hit)
}