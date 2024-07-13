use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use common::simple_rule::{NamedSimpleRule, SimpleRuleBorder, SimpleRulePart, SimpleRulePartValue, SimpleRuleSeq, WrappedSimpleRulePart};

use parser::parsing::simple_rule::error::SimpleRuleParseError;

fn parse_rule(s: &str) -> Result<NamedSimpleRule, SimpleRuleParseError> {
    parser::parse_simple_rule(s)
}

fn parse_part(kw: &str, val: &str) -> WrappedSimpleRulePart {
    parser::parse_simple_rule(format!("n: {kw} {val}").as_str())
        .unwrap()
        .rule
        .parts
        .remove(kw)
        .unwrap()
}

#[test]
fn name_correct() {
    let parsed = parse_rule("rule_name: player 1").unwrap();
    assert_eq!(parsed.name, "rule_name")
}

#[test]
fn name_incorrect() {
    let parsed = parse_rule(": player 1");
    assert!(parsed.is_err())
}

#[test]
fn player_asterisk() {
    let parsed = parse_part("player", "*");
    let expected = WrappedSimpleRulePart::Number(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn player_incorrect() {
    let parsed = parse_rule("n: player abc");
    assert!(parsed.is_err())
}

#[test]
fn player_any_asterisk() {
    let parsed = parse_part("player", "any(*)");
    let expected = WrappedSimpleRulePart::Number(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn player_number() {
    let parsed = parse_part("player", "3");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(3)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn player_any_numbers() {
    let parsed = parse_part("player", "any(3,4,5)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(3),
            SimpleRulePartValue::exact(4),
            SimpleRulePartValue::exact(5)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn action_number() {
    let parsed = parse_part("did", "2");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(2)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn action_incorrect() {
    let parsed = parse_rule("n: did abc");
    assert!(parsed.is_err())
}

#[test]
fn objectlike_asterisk() {
    let parsed = parse_part("with", "*");
    let expected = WrappedSimpleRulePart::Number(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_any_asterisk() {
    let parsed = parse_part("with", "any(*)");
    let expected = WrappedSimpleRulePart::Number(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_any_number() {
    let parsed = parse_part("with", "any(1)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_all_number() {
    let parsed = parse_part("with", "all(1)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_seq_number() {
    let parsed = parse_part("with", "seq(1)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Order, vec!(
            SimpleRulePartValue::exact(1)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_numbers() {
    let parsed = parse_part("with", "1,2,3,4");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3),
            SimpleRulePartValue::exact(4),
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_any_numbers() {
    let parsed = parse_part("with", "any(1,2,3,4)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3),
            SimpleRulePartValue::exact(4),
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_all_numbers() {
    let parsed = parse_part("with", "all(1,2,3,4)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3),
            SimpleRulePartValue::exact(4),
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_all_asterisk() {
    let parsed = parse_part("with", "all(*)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!())
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_seq_numbers() {
    let parsed = parse_part("with", "seq(1,2,3)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Order, vec!(
            SimpleRulePartValue::exact(1),
            SimpleRulePartValue::exact(2),
            SimpleRulePartValue::exact(3),
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_seq_asterisk() {
    let parsed = parse_part("with", "seq(*)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Order, vec!())
    );
    assert_eq!(parsed, expected)
}

#[test]
fn objectlike_incorrect() {
    let parsed = parse_rule("n: with abc");
    assert!(parsed.is_err())
}

#[test]
fn time_asterisk() {
    let parsed = parse_part("at", "*");
    let expected = WrappedSimpleRulePart::Time(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn time_any_asterisk() {
    let parsed = parse_part("at", "any(*)");
    let expected = WrappedSimpleRulePart::Time(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn time_time() {
    let parsed = parse_part("at", "10:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_any_time() {
    let parsed = parse_part("at", "any(10:00)");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_all_time() {
    let parsed = parse_part("at", "all(12:00)");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("12:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_times() {
    let parsed = parse_part("at", "10:00,11:00,23:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("11:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("23:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_any_times() {
    let parsed = parse_part("at", "any(10:00,11:00,23:00)");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("11:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("23:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_all_times() {
    let parsed = parse_part("at", "all(10:00,11:00,23:00)");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveTime::from_str("10:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("11:00").unwrap()),
            SimpleRulePartValue::exact(NaiveTime::from_str("23:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_incorrect() {
    let parsed = parse_rule("n: at 12-00");
    assert!(parsed.is_err())
}

#[test]
fn time_less() {
    let parsed = parse_part("at", "<10:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Less, NaiveTime::from_str("10:00").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_less_eq() {
    let parsed = parse_part("at", "<=10:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::LessEq, NaiveTime::from_str("10:00").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_more() {
    let parsed = parse_part("at", ">10:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, NaiveTime::from_str("10:00").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_more_eq() {
    let parsed = parse_part("at", ">=10:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::GreaterEq, NaiveTime::from_str("10:00").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_between() {
    let parsed = parse_part("at", "10:00..12:00");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(
                SimpleRuleBorder::Between,
                NaiveTime::from_str("10:00").unwrap(),
                Some(NaiveTime::from_str("12:00").unwrap())
            )
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn time_mixed() {
    let parsed = parse_part("at", "all(10:00..12:00,>15:00,23:00)");
    let expected = WrappedSimpleRulePart::Time(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::new(
                SimpleRuleBorder::Between,
                NaiveTime::from_str("10:00").unwrap(),
                Some(NaiveTime::from_str("12:00").unwrap())
            ),
            SimpleRulePartValue::new(
                SimpleRuleBorder::Greater,
                NaiveTime::from_str("15:00").unwrap(),
                None
            ),
            SimpleRulePartValue::exact(NaiveTime::from_str("23:00").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_asterisk() {
    let parsed = parse_part("on", "*");
    let expected = WrappedSimpleRulePart::Date(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn date_any_asterisk() {
    let parsed = parse_part("on", "any(*)");
    let expected = WrappedSimpleRulePart::Date(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn date_date() {
    let parsed = parse_part("on", "2000.01.01");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_any_date() {
    let parsed = parse_part("on", "any(2000.01.01)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_all_date() {
    let parsed = parse_part("on", "all(2000.01.01)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_dates() {
    let parsed = parse_part("on", "2000.01.01,2001.02.02,2003.10.10");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2001-02-02").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2003-10-10").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_any_dates() {
    let parsed = parse_part("on", "any(2000.01.01,2001.02.02,2003.10.10)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2001-02-02").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2003-10-10").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_all_dates() {
    let parsed = parse_part("on", "all(2000.01.01,2001.02.02,2003.10.10)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::exact(NaiveDate::from_str("2000-01-01").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2001-02-02").unwrap()),
            SimpleRulePartValue::exact(NaiveDate::from_str("2003-10-10").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_streak() {
    let parsed = parse_part("on", "streak(3)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Streak(3), vec!(
            SimpleRulePartValue::exact(NaiveDate::default()),
            SimpleRulePartValue::exact(NaiveDate::default()),
            SimpleRulePartValue::exact(NaiveDate::default())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_streak_incorrect() {
    let parsed = parse_rule("n: on streak(0)");
    assert!(parsed.is_err())
}

#[test]
fn date_selected_number() {
    let parsed = parse_part("on", "every(3)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Selected(3), vec!())
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_selected_number_incorrect() {
    let parsed = parse_rule("n: on every(9)");
    assert!(parsed.is_err())
}

#[test]
fn date_selected_text() {
    let parsed = parse_part("on", "every(MONDAY)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Selected(1), vec!())
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_selected_text_incorrect() {
    let parsed = parse_rule("n: on every(COOKIE)");
    assert!(parsed.is_err())
}

#[test]
fn date_incorrect() {
    let parsed = parse_rule("n: on 2000-01-01");
    assert!(parsed.is_err())
}

#[test]
fn date_less() {
    let parsed = parse_part("on", "<2000.01.01");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Less, NaiveDate::from_str("2000-01-01").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_less_eq() {
    let parsed = parse_part("on", "<=2000.01.01");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::LessEq, NaiveDate::from_str("2000-01-01").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_more() {
    let parsed = parse_part("on", ">2000.01.01");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, NaiveDate::from_str("2000-01-01").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_more_eq() {
    let parsed = parse_part("on", ">=2000.01.01");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::GreaterEq, NaiveDate::from_str("2000-01-01").unwrap(), None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_between() {
    let parsed = parse_part("on", "2000.01.01..2010.12.12");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(
                SimpleRuleBorder::Between,
                NaiveDate::from_str("2000-01-01").unwrap(),
                Some(NaiveDate::from_str("2010-12-12").unwrap())
            )
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn date_mixed() {
    let parsed = parse_part("on", "all(2000.01.01..2010.12.12,>2012.01.01,1999.06.06)");
    let expected = WrappedSimpleRulePart::Date(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::new(
                SimpleRuleBorder::Between,
                NaiveDate::from_str("2000-01-01").unwrap(),
                Some(NaiveDate::from_str("2010-12-12").unwrap())
            ),
            SimpleRulePartValue::new(
                SimpleRuleBorder::Greater,
                NaiveDate::from_str("2012-01-01").unwrap(),
                None
            ),
            SimpleRulePartValue::exact(NaiveDate::from_str("1999-06-06").unwrap())
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_asterisk() {
    let parsed = parse_part("achieving", "*");
    let expected = WrappedSimpleRulePart::Number(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn result_any_asterisk() {
    let parsed = parse_part("achieving", "any(*)");
    let expected = WrappedSimpleRulePart::Number(SimpleRulePart::empty());
    assert_eq!(parsed, expected)
}

#[test]
fn result_number() {
    let parsed = parse_part("achieving", "100");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(100)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_any_number() {
    let parsed = parse_part("achieving", "any(100)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(100)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_numbers() {
    let parsed = parse_part("achieving", "10,50,75");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(10),
            SimpleRulePartValue::exact(50),
            SimpleRulePartValue::exact(75)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_any_numbers() {
    let parsed = parse_part("achieving", "any(10,50,75)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::exact(10),
            SimpleRulePartValue::exact(50),
            SimpleRulePartValue::exact(75)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_incorrect() {
    let parsed = parse_rule("n: achieving abc");
    assert!(parsed.is_err())
}

#[test]
fn result_less() {
    let parsed = parse_part("achieving", "<100");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Less, 100, None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_less_eq() {
    let parsed = parse_part("achieving", "<=100");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::LessEq, 100, None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_more() {
    let parsed = parse_part("achieving", ">100");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::Greater, 100, None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_more_eq() {
    let parsed = parse_part("achieving", ">=100");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(SimpleRuleBorder::GreaterEq, 100, None)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_between() {
    let parsed = parse_part("achieving", "50..100");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::Any, vec!(
            SimpleRulePartValue::new(
                SimpleRuleBorder::Between,
                50,
                Some(100)
            )
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn result_mixed() {
    let parsed = parse_part("achieving", "all(10..25,>50,99)");
    let expected = WrappedSimpleRulePart::Number(
        SimpleRulePart::new(SimpleRuleSeq::All, vec!(
            SimpleRulePartValue::new(
                SimpleRuleBorder::Between,
                10,
                Some(25)
            ),
            SimpleRulePartValue::new(
                SimpleRuleBorder::Greater,
                50,
                None
            ),
            SimpleRulePartValue::exact(99)
        ))
    );
    assert_eq!(parsed, expected)
}

#[test]
fn repeat_number() {
    let parsed = parse_rule("n: repeat 3 player 1").unwrap();
    assert_eq!(parsed.rule.iterations, 3)
}

#[test]
fn repeat_incorrect() {
    let parsed = parse_rule("n: repeat 0 player 1");
    assert!(parsed.is_err())
}

#[test]
fn repeat_incorrect_noarg() {
    let parsed = parse_rule("n: repeat player 1");
    assert!(parsed.is_err())
}

#[test]
fn repeat_none() {
    let parsed = parse_rule("n: player 1").unwrap();
    assert_eq!(parsed.rule.iterations, 1)
}

#[test]
fn repeat_infite() {
    let parsed = parse_rule("n: repeat + player 1").unwrap();
    assert_eq!(parsed.rule.iterations, -1)
}

#[test]
fn rule() {
    let parsed = parse_rule("name: player 1 did 2 with 3 in 4 of 5 on 2000.01.01 at 12:00 achieving 100 repeat 1");
    assert!(parsed.is_ok())
}

#[test]
fn rule_incorrect() {
    let parsed = parse_rule("name: player 1 did 2 with 3 in 4 of 5 on at 12:00 achieving 100 repeat 1");
    assert!(parsed.is_err())
}