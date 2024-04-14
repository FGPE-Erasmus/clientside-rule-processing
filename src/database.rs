use std::collections::HashMap;

use crate::core::complex_rule::ComplexRule;
use crate::core::event::Event;
use crate::core::result::{Kind, Result};
use crate::core::rule::Rule;

pub struct Database {
    rules: Vec<Rule>,
    complex_rules: Vec<ComplexRule>,
    results: HashMap<String, Result>
}

impl Database {
    pub fn new(rules: Vec<Rule>, complex_rules: Vec<ComplexRule>, results: Vec<Result>) -> Self {
        let mut res_map = HashMap::new();
        results
            .into_iter()
            .for_each(|r| {
            res_map.insert(r.name().to_string(), r);
        });
        Self {
            rules,
            complex_rules,
            results: res_map
        }
    }
    pub fn process(&mut self, event: &Event) -> ProcessingResult {
        let mut fired_rules = vec!();
        let mut completed_rules = vec!();
        let mut results = vec!();
        self.process_rules(event, &mut fired_rules, &mut completed_rules);
        self.process_complex_rules(event, &mut fired_rules, &mut completed_rules);
        self.process_results(&completed_rules, &mut results);
        ProcessingResult {
            fired_rules,
            completed_rules,
            results
        }
    }
    fn process_rules(&mut self, event: &Event, fired_rules: &mut Vec<String>, completed_rules: &mut Vec<String>) {
        for rule in self.rules.iter_mut() {
            if rule.is_completed() {
                continue;
            }
            let fired = rule.fired(event);
            if fired.is_none() {
                continue;
            }
            fired_rules.push(rule.name().to_string());
            let completed = rule.completed(event, fired.unwrap());
            if completed {
                completed_rules.push(rule.name().to_string());
                let repeat = rule.repeat();
                if repeat == -1 {
                    rule.reset(-1)
                } else if repeat > 0 {
                    rule.reset(repeat - 1)
                } else {
                    rule.set_completed();
                }
            }
        }
    }
    fn process_complex_rules(&mut self, event: &Event, fired_rules: &mut Vec<String>, completed_rules: &mut Vec<String>) {
        let mut completed_complex_rules = vec!();
        for rule in completed_rules.iter_mut() {
            for complex_rule in self.complex_rules.iter_mut() {
                if complex_rule.is_completed() {
                    continue;
                }
                let fired = complex_rule.fired(rule, &event.date());
                if fired.is_none() {
                    continue;
                }
                fired_rules.push(complex_rule.name().to_string());
                let completed = complex_rule.completed(&fired, &event.date());
                if completed {
                    completed_complex_rules.push(complex_rule.name().to_string());
                    if complex_rule.repeat() {
                        complex_rule.reset();
                    } else {
                        complex_rule.set_completed();
                    }
                }
            }
        }
        completed_rules.append(&mut completed_complex_rules);
    }
    fn process_results(&mut self, completed_rules: &Vec<String>, results: &mut Vec<ResultDetails>) {
        for completed_rule in completed_rules {
            let result = self.results.get_mut(completed_rule.as_str());
            if result.is_none() {
                continue;
            }
            let result = result.unwrap();
            if result.is_completed() {
                continue;
            }
            let proc_res = result.process();
            let mut res_details = ResultDetails {
                name: completed_rule.to_owned(),
                kind: proc_res.1.to_string(),
                completed: proc_res.2,
                data: proc_res.0,
                restart_data: None
            };
            if let Kind::Restart = proc_res.1 {
                res_details.restart_data = Some(vec!());
                for to_restart in &res_details.data {
                    let mut restarted = false;
                    let simple_pos = self.rules
                        .iter()
                        .position(|r| r.name().eq(to_restart.as_str()));
                    if simple_pos.is_some() {
                        let rule = self.rules.get_mut(simple_pos.unwrap()).unwrap();
                        rule.reset(0);
                        restarted = true;
                    } else {
                        let complex_pos = self.complex_rules
                            .iter()
                            .position(|r| r.name().eq(to_restart.as_str()));
                        if complex_pos.is_some() {
                            let rule = self.complex_rules.get_mut(complex_pos.unwrap()).unwrap();
                            rule.reset();
                            restarted = true;
                        }
                    }
                    res_details.restart_data.as_mut().unwrap().push((to_restart.to_string(), restarted));
                }
            }
            results.push(res_details);
            if proc_res.2 {
                if result.repeat() {
                    result.reset();
                } else {
                    result.set_completed()
                }
            }
        }
    }
}

pub struct ProcessingResult {
    fired_rules: Vec<String>,
    completed_rules: Vec<String>,
    results: Vec<ResultDetails>
}

pub struct ResultDetails {
    name: String,
    kind: String,
    completed: bool,
    data: Vec<String>,
    restart_data: Option<Vec<(String, bool)>>
}

impl ResultDetails {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn kind(&self) -> &str {
        &self.kind
    }
    pub fn completed(&self) -> bool {
        self.completed
    }
    pub fn data(&self) -> &Vec<String> {
        &self.data
    }
    pub fn restart_data(&self) -> &Option<Vec<(String, bool)>> {
        &self.restart_data
    }
}

impl ProcessingResult {
    pub fn fired_rules(&self) -> &Vec<String> {
        &self.fired_rules
    }
    pub fn completed_rules(&self) -> &Vec<String> {
        &self.completed_rules
    }
    pub fn results(&self) -> &Vec<ResultDetails> {
        &self.results
    }
}