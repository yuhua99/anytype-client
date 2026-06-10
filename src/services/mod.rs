//! Business workflows shared by CLI commands.
//!
//! Service modules should own multi-step workflows and request construction.
//! They should not render output or perform CLI parsing.

pub(crate) mod objects;

/// How a user-supplied identifier was resolved to an entity.
///
/// `Fuzzy` means a unique partial (substring) match was selected; callers
/// surface a stderr notice so destructive commands are never silently
/// redirected to an unexpected entity.
#[derive(Debug)]
pub(crate) enum Match<T> {
    Exact(T),
    Fuzzy(T),
}
pub(crate) mod property_resolution;
pub(crate) mod search;
pub(crate) mod space_resolution;
pub(crate) mod tag_resolution;
pub(crate) mod type_resolution;
