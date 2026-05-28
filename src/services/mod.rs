//! Business workflows shared by CLI commands.
//!
//! Service modules should own multi-step workflows and request construction.
//! They should not render output or perform CLI parsing.

pub mod search;
