use std::error::Error;
use std::fmt::Display;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::str::FromStr;

use wasm_bindgen::prelude::wasm_bindgen;

pub use config::Config;

use crate::core::complex_rule::ComplexRule;
use crate::core::event::Event;
use crate::core::rule::Rule;
use crate::database::{Database, ResultDetails};

mod config;
mod core;
mod database;

#[wasm_bindgen]
pub fn run_parse(event: Event, rules: String,
                 complex_rules: String, results: String) -> (String, Vec<ResultDetails>) {
    let rules: Result<Vec<Rule>, _> = parse_data_from_str(rules);
    if let Err(msg) = rules {
        eprintln!("fatal error during rule parsing, details: {msg}");
        return (String::new(), vec!())
    }
    let complex_rules: Result<Vec<ComplexRule>, _> = parse_data_from_str(complex_rules);
    if let Err(msg) = complex_rules {
        eprintln!("fatal error during complex rule parsing, details: {msg}");
        return (String::new(), vec!())
    }
    let results: Result<Vec<core::result::Result>, _> = parse_data_from_str(results);
    if let Err(msg) = results {
        eprintln!("fatal error during result parsing, details: {msg}");
        return (String::new(), vec!())
    }
    let mut db = Database::new(rules.unwrap(), complex_rules.unwrap(), results.unwrap());
    let res = db.process(&event);
    (db.save(), res.into_results())
}

#[wasm_bindgen]
pub fn run_cache(event: Event, game_state: String) -> (String, Vec<ResultDetails>) {
    let mut db = Database::load(game_state);
    let res = db.process(&event);
    (db.save(), res.into_results())
}

fn parse_data_from_str<T>(s: String) -> Result<Vec<T>, Box<dyn Error>>
    where T: FromStr, <T as FromStr>::Err: Display {
    Ok(BufReader::new(s)
        .lines()
        .filter_map(|res| res.ok())
        .enumerate()
        .filter_map(|(mut i, line)| {
            i += 1;
            let res = line.parse();
            if let Err(err) = &res {
                eprintln!("could not parse file {path} line {i}, reason: {err}");
            }
            res.ok()
        })
        .collect())
}