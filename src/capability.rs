//! Capability Definition and Registry.
//!
//! Capability Registry maintains the authoritative list of registered capabilities.
//! Unknown capabilities MUST be denied by default.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub parameters: Vec<CapabilityParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParam {
    pub name: String,
    pub param_type: String,
    pub required: bool,
}

impl Capability {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            enabled: true,
            parameters: Vec::new(),
        }
    }

    pub fn with_params(mut self, params: Vec<CapabilityParam>) -> Self {
        self.parameters = params;
        self
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }
}

#[derive(Default)]
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Capability>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, capability: Capability) {
        self.capabilities
            .insert(capability.name.clone(), capability);
    }

    pub fn get(&self, name: &str) -> Option<&Capability> {
        self.capabilities.get(name)
    }

    pub fn is_registered(&self, name: &str) -> bool {
        self.capabilities.contains_key(name)
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        self.capabilities
            .get(name)
            .map(|c| c.enabled)
            .unwrap_or(false)
    }

    pub fn list(&self) -> Vec<&Capability> {
        self.capabilities.values().collect()
    }

    pub fn enable(&mut self, name: &str) -> bool {
        if let Some(cap) = self.capabilities.get_mut(name) {
            cap.enable();
            true
        } else {
            false
        }
    }

    pub fn disable(&mut self, name: &str) -> bool {
        if let Some(cap) = self.capabilities.get_mut(name) {
            cap.disable();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_capability() {
        let mut registry = CapabilityRegistry::new();
        let cap = Capability::new("fs.read", "Read files from filesystem");
        registry.register(cap);

        assert!(registry.is_registered("fs.read"));
        assert!(registry.is_enabled("fs.read"));
    }

    #[test]
    fn test_disable_capability() {
        let mut registry = CapabilityRegistry::new();
        let cap = Capability::new("shell", "Execute shell commands");
        registry.register(cap);

        registry.disable("shell");
        assert!(!registry.is_enabled("shell"));
    }

    #[test]
    fn test_unknown_capability() {
        let registry = CapabilityRegistry::new();
        assert!(!registry.is_registered("unknown"));
    }
}
