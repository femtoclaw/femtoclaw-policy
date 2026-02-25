//! FemtoClaw Policy Engine Library.
//!
//! Provides capability gating and authorization enforcement according to
//! FemtoClaw Capability Authorization Specification (FC-05).
//!
//! # Architecture
//!
//! - [`CapabilityRegistry`] - maintains registered capabilities
//! - [`PolicyEngine`] - evaluates authorization rules
//! - [`CapabilityGate`] - enforces authorization decisions
//! - [`Decision`] - authorization decision types

pub mod capability;
pub mod gate;
pub mod policy;

pub use capability::{Capability, CapabilityRegistry};
pub use gate::CapabilityGate;
pub use policy::{Policy, PolicyEngine, Rule};
pub use gate::Decision;
