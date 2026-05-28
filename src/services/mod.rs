//! Business workflows shared by CLI commands.
//!
//! Service modules should own multi-step workflows and request construction.
//! They should not render output or perform CLI parsing.

pub mod objects;
pub(crate) mod property_resolution;
pub mod search;
pub(crate) mod space_resolution;
pub(crate) mod tag_resolution;
