//! FemtoClaw Policy Engine Library.
//!
//! Provides capability gating and authorization enforcement according to
//! FemtoClaw Capability Authorization Specification (FC-05).

pub mod capability;
pub mod evaluator;
pub mod policy;

pub use capability::Capability;
pub use evaluator::Evaluator;
pub use policy::{Effect, Policy, Rule};
