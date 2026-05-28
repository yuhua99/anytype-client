mod auth;
mod client;
mod files;
mod lists;
mod members;
mod objects;
mod properties;
mod search;
mod spaces;
mod tags;
mod types;

pub use client::{AnytypeClient, PageOptions};

// Centralized API endpoint path builders.
// Owned by the api layer; used only by endpoint method implementations.
// Narrow scope for initial centralization: object and search paths.

fn global_search_path() -> &'static str {
    "/search"
}

fn space_search_path(space_id: &str) -> String {
    format!("/spaces/{space_id}/search")
}

fn space_objects_path(space_id: &str) -> String {
    format!("/spaces/{space_id}/objects")
}

fn space_object_path(space_id: &str, object_id: &str) -> String {
    format!("/spaces/{space_id}/objects/{object_id}")
}

fn space_object_path_with_format(space_id: &str, object_id: &str, format: &str) -> String {
    format!("/spaces/{space_id}/objects/{object_id}?format={format}")
}
