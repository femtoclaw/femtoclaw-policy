//! Policy Evaluation Engine.

use crate::policy::{Effect, Policy, Rule};

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, policy: &Policy, resource: &str, action: &str) -> bool {
        for rule in &policy.rules {
            if rule.resource == resource && rule.action == action {
                return matches!(rule.effect, Effect::Allow);
            }
        }
        false
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
