use std::collections::HashMap;
use common::compound_rule::{CompoundRule, NamedCompoundRule};
use common::event::Event;
use common::rule_result::{NamedRuleResult, RuleResult, RuleResultKind};
use common::simple_rule::{NamedSimpleRule, SimpleRule};
use serde::{Deserialize, Serialize};

mod processing;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub enabled_simple_rules: HashMap<String, SimpleRule>,
    pub enabled_compound_rules: HashMap<String, CompoundRule>,
    pub enabled_rule_results: HashMap<String, RuleResult>,
    pub disabled_simple_rules: HashMap<String, SimpleRule>,
    pub disabled_compound_rules: HashMap<String, CompoundRule>,
    pub disabled_rule_results: HashMap<String, RuleResult>,
}

impl State {
    pub fn new(simple_rules: Vec<NamedSimpleRule>,
           compound_rules: Vec<NamedCompoundRule>,
           rule_results: Vec<NamedRuleResult>) -> Self {
        Self {
            enabled_simple_rules: simple_rules.into_iter()
                .map(|v| (v.name, v.rule))
                .collect(),
            enabled_compound_rules: compound_rules.into_iter()
                .map(|v| (v.name, v.rule))
                .collect(),
            enabled_rule_results: rule_results.into_iter()
                .map(|v| (v.name, v.res))
                .collect(),
            disabled_simple_rules: HashMap::new(),
            disabled_compound_rules: HashMap::new(),
            disabled_rule_results: HashMap::new()
        }
    }
    pub fn save(&self) -> String {
        serde_json::to_string(self)
            .expect("should be able to serialize state object")
    }
    pub fn load(data: &str) -> Self {
        serde_json::from_str(data)
            .expect("should be able to deserialize state object")
    }
    pub fn update(&mut self, event: &Event) -> Vec<(RuleResultKind, Vec<String>)> {
        let mut completed_rules = processing::process_simple_rules(
            &mut self.enabled_simple_rules, &mut self.disabled_simple_rules, event
        );
        let completed_compound_rules = processing::process_compound_rules(
            &mut self.enabled_compound_rules, &mut self.disabled_compound_rules,
            &completed_rules, event
        );
        completed_rules.extend(completed_compound_rules);
        processing::process_rule_results(self, completed_rules)
    }
}