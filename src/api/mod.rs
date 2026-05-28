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
// Grouped by domain for maintainability; currently covers objects, search, and types.

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

// Types domain paths
fn space_types_path(space_id: &str) -> String {
    format!("/spaces/{space_id}/types")
}

fn space_type_path(space_id: &str, type_id: &str) -> String {
    format!("/spaces/{space_id}/types/{type_id}")
}

fn space_type_templates_path(space_id: &str, type_id: &str) -> String {
    format!("/spaces/{space_id}/types/{type_id}/templates")
}

fn space_type_template_path(space_id: &str, type_id: &str, template_id: &str) -> String {
    format!("/spaces/{space_id}/types/{type_id}/templates/{template_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centralized_object_and_search_paths_match_expected() {
        assert_eq!(global_search_path(), "/search");
        assert_eq!(space_search_path("s1"), "/spaces/s1/search");
        assert_eq!(space_objects_path("s1"), "/spaces/s1/objects");
        assert_eq!(space_object_path("s1", "o1"), "/spaces/s1/objects/o1");
        assert_eq!(
            space_object_path_with_format("s1", "o1", "md"),
            "/spaces/s1/objects/o1?format=md"
        );
        // Types domain (broadened centralization)
        assert_eq!(space_types_path("s1"), "/spaces/s1/types");
        assert_eq!(space_type_path("s1", "t1"), "/spaces/s1/types/t1");
        assert_eq!(
            space_type_templates_path("s1", "t1"),
            "/spaces/s1/types/t1/templates"
        );
        assert_eq!(
            space_type_template_path("s1", "t1", "tpl1"),
            "/spaces/s1/types/t1/templates/tpl1"
        );
    }
}
