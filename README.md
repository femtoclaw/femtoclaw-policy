# FemtoClaw Policy

[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Status](https://img.shields.io/badge/Status-Normative-green.svg)]()

FemtoClaw Policy Engine — capability gating and authorization enforcement.

## Overview

`femtoclaw-policy` provides the authorization and capability gating layer for the FemtoClaw Industrial Agent Runtime. It implements the policy evaluation engine according to the [FemtoClaw Capability Authorization Specification (FC-05)](../femtoclaw-spec/05-FemtoClaw_Capability_Authorization_and_Policy_Specification.md).

This library enforces deny-by-default security by requiring explicit capability authorization before any operating system interaction.

## Features

- **Capability Definitions**: Define and register executable capabilities
- **Policy Engine**: Evaluate authorization decisions based on policies
- **Rule-based Authorization**: Support for Allow/Deny rules with conditions
- **Resource Gating**: Control access to filesystem, network, process, and other resources

## Architecture

```
┌─────────────────────────────────────────────┐
│         Protocol Validation                │
└─────────────────┬───────────────────────────┘
                  │ Validated Tool Call
                  ▼
┌─────────────────────────────────────────────┐
│         femtoclaw-policy                   │
│  ┌─────────────────────────────────────┐   │
│  │  Capability Registry                │   │
│  │  - filesystem.read                  │   │
│  │  - filesystem.write                 │   │
│  │  - network.http                     │   │
│  │  - process.spawn                    │   │
│  └─────────────────────────────────────┘   │
│  ┌─────────────────────────────────────┐   │
│  │  Policy Evaluator                   │   │
│  │  - Rule matching                    │   │
│  │  - Condition evaluation             │   │
│  │  - Authorization decision          │   │
│  └─────────────────────────────────────┘   │
└─────────────────┬───────────────────────────┘
                  │ Authorization Decision
                  ▼
┌─────────────────────────────────────────────┐
│         Capability Execution Layer          │
└─────────────────────────────────────────────┘
```

## Installation

```toml
[dependencies]
femtoclaw-policy = "1.0"
```

## Usage

```rust
use femtoclaw_policy::{Capability, Policy, Evaluator, Effect, Rule};

// Define a capability
let capability = Capability {
    name: "filesystem.read".to_string(),
    description: "Read files from the filesystem".to_string(),
    parameters: vec![],
};

// Create a policy with allow rules
let policy = Policy {
    name: "default".to_string(),
    version: "1.0".to_string(),
    rules: vec![
        Rule {
            effect: Effect::Allow,
            principal: "*".to_string(),
            resource: "filesystem.read".to_string(),
            action: "execute".to_string(),
            conditions: vec![],
        },
    ],
};

// Evaluate authorization
let evaluator = Evaluator::new();
let allowed = evaluator.evaluate(&policy, "filesystem.read", "execute");

if allowed {
    println!("Capability authorized");
} else {
    println!("Capability denied");
}
```

## Modules

- `capability` — Capability definitions and parameters
- `policy` — Policy, Rule, Effect, and Condition types
- `evaluator` — Policy evaluation engine

## Requirements

- Rust 1.75 or later
- serde 1.x
- serde_json 1.x
- thiserror 1.x

## Related Specifications

- [FC-04: Capability (Claw) Specification](../femtoclaw-spec/04-FemtoClaw_Capability_Claw_Specification.md)
- [FC-05: Capability Authorization](../femtoclaw-spec/05-FemtoClaw_Capability_Authorization_and_Policy_Specification.md)
- [FC-07: Security Architecture](../femtoclaw-spec/07-FemtoClaw_Security_Architecture_Specification.md)

## License

Copyright 2026 FemtoClaw

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
