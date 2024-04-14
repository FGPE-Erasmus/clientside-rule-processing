use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, stdout};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
pub use config::Config;
use crate::core::complex_rule::ComplexRule;
use crate::core::event::Event;
use crate::core::rule::Rule;
use crate::database::{Database, ProcessingResult};

mod config;
mod core;
mod database;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut rules: Vec<Rule> = parse_data(&config.rules())?;
    include_debug_data(&config, &mut rules);
    let complex_rules: Vec<ComplexRule> = parse_data(&config.complex_rules())?;
    let events: Vec<Event> = parse_data(&config.events())?;
    let results: Vec<core::result::Result> = parse_data(&config.results())?;
    let mut db = Database::new(rules, complex_rules, results);
    let res = events
        .into_iter()
        .map(|e| (e.to_string(), db.process(&e)))
        .collect();
    display_results(res);
    Ok(())
}

fn include_debug_data(config: &Config, rules: &mut Vec<Rule>) {
    rules
        .iter_mut()
        .for_each(|r| r.include_debug_data(config))
}

fn display_results(results: Vec<(String, ProcessingResult)>) {
    let mut out_lock = stdout().lock();
    results.into_iter().for_each(|res| {
         writeln!(out_lock, "{}\n\tfired: {:?}\n\tcompleted: {:?}", res.0,
                  res.1.fired_rules(), res.1.completed_rules())
             .expect("should be able to print in normal circumstances");
        res.1.results()
            .into_iter()
            .for_each(|r| {
                writeln!(out_lock, "\"{}\" result ({}) fired; completed: {}, data: {:?}", r.name(),
                         r.kind(), r.completed(), r.data())
                    .expect("should be able to print in normal circumstances");
                if let Some(data) = r.restart_data() {
                    data
                        .into_iter()
                        .for_each(|d| {
                            writeln!(out_lock, "rule {} found and restarted: {}", d.0, d.1)
                                .expect("should be able to print in normal circumstances");
                        })
                }
            });
        writeln!(out_lock, "")
            .expect("should be able to print in normal circumstances");
    });
}

fn parse_data<P: AsRef<Path> + Display, T>(path: &P) -> Result<Vec<T>, Box<dyn Error>>
    where T: FromStr, <T as FromStr>::Err: Display {
    Ok(BufReader::new(File::open(path)?)
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