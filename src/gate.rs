//! Capability Gate.
//!
//! Capability Gate enforces authorization decisions. It is the final execution
//! authorization authority that verifies:
//! 1. Capability exists in registry
//! 2. Capability is enabled
//! 3. Policy engine permits execution

use crate::capability::{Capability, CapabilityRegistry};
use crate::policy::{Effect, PolicyEngine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Authorized,
    DeniedCapabilityNotFound,
    DeniedCapabilityDisabled,
    DeniedPolicyViolation,
}

impl Decision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Decision::Authorized)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Decision::Authorized => "AUTHORIZED",
            Decision::DeniedCapabilityNotFound => "DENIED_CAPABILITY_NOT_FOUND",
            Decision::DeniedCapabilityDisabled => "DENIED_CAPABILITY_DISABLED",
            Decision::DeniedPolicyViolation => "DENIED_POLICY_VIOLATION",
        }
    }
}

impl std::fmt::Display for Decision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct CapabilityGate {
    registry: CapabilityRegistry,
    engine: PolicyEngine,
}

impl CapabilityGate {
    pub fn new() -> Self {
        Self {
            registry: CapabilityRegistry::new(),
            engine: PolicyEngine::new().with_default_effect(Effect::Deny),
        }
    }

    pub fn with_registry(mut self, registry: CapabilityRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub fn with_engine(mut self, engine: PolicyEngine) -> Self {
        self.engine = engine;
        self
    }

    pub fn register_capability(&mut self, capability: Capability) {
        self.registry.register(capability);
    }

    pub fn add_policy(&mut self, policy: crate::policy::Policy) {
        self.engine.add_policy(policy);
    }

    pub fn authorize(&self, tool: &str, args: &serde_json::Value) -> Decision {
        if !self.registry.is_registered(tool) {
            return Decision::DeniedCapabilityNotFound;
        }

        if !self.registry.is_enabled(tool) {
            return Decision::DeniedCapabilityDisabled;
        }

        let effect = self.engine.evaluate(tool, "execute", args);

        match effect {
            Effect::Allow => Decision::Authorized,
            Effect::Deny => Decision::DeniedPolicyViolation,
        }
    }

    pub fn check(&self, tool: &str) -> bool {
        self.authorize(tool, &serde_json::json!({})).is_allowed()
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;
    use crate::policy::{Policy, Rule};

    #[test]
    fn test_unknown_capability() {
        let gate = CapabilityGate::new();
        let result = gate.authorize("unknown", &serde_json::json!({}));
        assert_eq!(result, Decision::DeniedCapabilityNotFound);
    }

    #[test]
    fn test_disabled_capability() {
        let mut gate = CapabilityGate::new();
        gate.register_capability(Capability::new("shell", "Shell commands"));
        gate.registry.disable("shell");

        let result = gate.authorize("shell", &serde_json::json!({}));
        assert_eq!(result, Decision::DeniedCapabilityDisabled);
    }

    #[test]
    fn test_allowed_by_policy() {
        let mut gate = CapabilityGate::new();
        gate.register_capability(Capability::new("fs.read", "Read files"));

        let policy = Policy::new("default", "1.0").with_rule(Rule::allow("fs.read"));
        gate.add_policy(policy);

        let result = gate.authorize("fs.read", &serde_json::json!({}));
        assert_eq!(result, Decision::Authorized);
    }

    #[test]
    fn test_denied_by_policy() {
        let mut gate = CapabilityGate::new();
        gate.register_capability(Capability::new("shell", "Shell commands"));

        let policy = Policy::new("default", "1.0").with_rule(Rule::deny("shell"));
        gate.add_policy(policy);

        let result = gate.authorize("shell", &serde_json::json!({}));
        assert_eq!(result, Decision::DeniedPolicyViolation);
    }
}
