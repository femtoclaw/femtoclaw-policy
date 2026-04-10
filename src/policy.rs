//! Policy Rule Definitions and Policy Engine.
//!
//! Policy Engine evaluates authorization rules to determine if execution is permitted.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub version: String,
    pub rules: Vec<Rule>,
}

impl Policy {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            rules: Vec::new(),
        }
    }

    pub fn with_rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub effect: Effect,
    pub principal: String,
    pub resource: String,
    pub action: String,
    pub conditions: Vec<Condition>,
}

impl Rule {
    pub fn allow(resource: impl Into<String>) -> Self {
        Self {
            effect: Effect::Allow,
            principal: "*".to_string(),
            resource: resource.into(),
            action: "execute".to_string(),
            conditions: Vec::new(),
        }
    }

    pub fn deny(resource: impl Into<String>) -> Self {
        Self {
            effect: Effect::Deny,
            principal: "*".to_string(),
            resource: resource.into(),
            action: "execute".to_string(),
            conditions: Vec::new(),
        }
    }

    pub fn with_conditions(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions = conditions;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub key: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Default)]
pub struct PolicyEngine {
    policies: HashMap<String, Policy>,
    default_effect: Effect,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default_effect(mut self, effect: Effect) -> Self {
        self.default_effect = effect;
        self
    }

    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.insert(policy.name.clone(), policy);
    }

    pub fn evaluate(&self, resource: &str, _action: &str, _args: &serde_json::Value) -> Effect {
        for policy in self.policies.values() {
            for rule in &policy.rules {
                if rule.resource == resource || rule.resource == "*" {
                    return rule.effect;
                }
            }
        }
        self.default_effect
    }

    pub fn load_from_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
        let policies: Vec<Policy> = serde_json::from_str(json)?;
        for policy in policies {
            self.add_policy(policy);
        }
        Ok(())
    }
}

impl Default for Effect {
    fn default() -> Self {
        Effect::Deny
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_rule() {
        let rule = Rule::allow("fs.read");
        assert_eq!(rule.effect, Effect::Allow);
        assert_eq!(rule.resource, "fs.read");
    }

    #[test]
    fn test_deny_rule() {
        let rule = Rule::deny("shell");
        assert_eq!(rule.effect, Effect::Deny);
    }

    #[test]
    fn test_policy_engine() {
        let mut engine = PolicyEngine::new().with_default_effect(Effect::Deny);

        let policy = Policy::new("default", "1.0")
            .with_rule(Rule::allow("fs.read"))
            .with_rule(Rule::allow("web.get"))
            .with_rule(Rule::deny("shell"));

        engine.add_policy(policy);

        assert_eq!(
            engine.evaluate("fs.read", "execute", &serde_json::json!({})),
            Effect::Allow
        );
        assert_eq!(
            engine.evaluate("shell", "execute", &serde_json::json!({})),
            Effect::Deny
        );
        assert_eq!(
            engine.evaluate("unknown", "execute", &serde_json::json!({})),
            Effect::Deny
        );
    }
}
