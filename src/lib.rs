use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, stdout};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
pub use config::Config;
use crate::core::event::Event;
use crate::core::rule::Rule;
use crate::database::{Database, ProcessingResult};

mod config;
mod core;
mod database;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut rules: Vec<Rule> = parse_data(config.rules())?;
    include_debug_data(&config, &mut rules);
    let events: Vec<Event> = parse_data(config.events())?;
    let mut db = Database::new(rules);
    let results = events
        .into_iter()
        .map(|e| (e.to_string(), db.process(&e)))
        .collect();
    display_results(results);
    Ok(())
}

fn include_debug_data(config: &Config, rules: &mut Vec<Rule>) {
    rules
        .iter_mut()
        .for_each(|r| r.include_debug_data(config))
}

fn display_results(results: Vec<(String, ProcessingResult)>) {
    let mut out_lock = stdout().lock();
    results
        .into_iter()
        .for_each(|res| {
            writeln!(out_lock, "{}\n\tfired: {:?}\n\tcompleted: {:?}\n", res.0,
                     res.1.fired_rules(), res.1.completed_rules())
                .expect("should be able to print in normal circumstances");
        })
}

fn parse_data<P: AsRef<Path>, T>(path: P) -> Result<Vec<T>, Box<dyn Error>>
    where T: FromStr, <T as FromStr>::Err: std::fmt::Display {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|line| {
            let res = line.parse();
            if let Err(err) = &res {
                eprintln!("could not parse, reason: {err}");
            }
            res.ok()
        })
        .collect())
}