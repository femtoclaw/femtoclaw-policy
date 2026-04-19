# 🛡️ FemtoClaw Policy Engine

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

The **FemtoClaw Policy Engine** is the deterministic authority that decides which capabilities an agent is permitted to execute. It implements the **Capability Authorization Specification (FC-05)** and serves as the primary barrier against unauthorized system interaction.

---

## 🧱 Authorization Model

FemtoClaw uses a strict **Deny-by-Default** security model. If an action is not explicitly permitted by an `Allow` rule, it is blocked.

### Key Concepts:
- **Capability Gating**: Policies are scoped to specific capabilities (e.g., `shell`, `fs.read`, `net`).
- **Effect Hierarchy**: `Deny` rules always take precedence over `Allow` rules. If a policy matches both, the action is denied.
- **Resource Context**: Decisions can be made based on the arguments provided to the capability (e.g., allow `fs.read` but only for the `/tmp` directory).
- **Audit Logging**: Every authorization decision (Allow or Deny) is recorded as an event in the audit trail.

---

## 📝 Usage & Policy Definition

Policies are defined programmatically and added to the runtime engine.

```rust
use femtoclaw_policy::{Policy, PolicyEngine, Rule};

// 1. Create a new policy engine
let mut engine = PolicyEngine::new();

// 2. Define a restrictive industrial policy
let policy = Policy::new("production-v1", "1.0")
    .with_rule(Rule::allow("shell"))  // Permit shell commands
    .with_rule(Rule::deny("fs.write")) // Explicitly block all writes
    .with_rule(Rule::allow("net"));   // Permit network access

// 3. Register the policy
engine.add_policy(policy);

// 4. Evaluate an execution request
let decision = engine.evaluate("shell", "execute", &serde_json::json!({
    "bin": "echo",
    "argv": ["hello"]
}));

if decision.is_allowed() {
    // Proceed to execution
}
```

---

## 📄 Related Specifications
- **[FC-05: Capability Authorization Specification](../femtoclaw-spec/05-FemtoClaw_Capability_Authorization_and_Policy_Specification.md)**
- **[FC-07: Security Architecture Specification](../femtoclaw-spec/07-FemtoClaw_Security_Architecture_Specification.md)**

Copyright © 2026 FemtoClaw Project.
