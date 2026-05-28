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
// Grouped by domain for maintainability; currently covers objects, search, types, properties, tags, files, lists, members, spaces, and auth.

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

// Properties domain paths
fn space_properties_path(space_id: &str) -> String {
    format!("/spaces/{space_id}/properties")
}

fn space_property_path(space_id: &str, property_id: &str) -> String {
    format!("/spaces/{space_id}/properties/{property_id}")
}

// Tags domain paths (nested under properties)
fn space_property_tags_path(space_id: &str, property_id: &str) -> String {
    format!("/spaces/{space_id}/properties/{property_id}/tags")
}

fn space_property_tag_path(space_id: &str, property_id: &str, tag_id: &str) -> String {
    format!("/spaces/{space_id}/properties/{property_id}/tags/{tag_id}")
}

// Files domain paths
fn space_files_path(space_id: &str) -> String {
    format!("/spaces/{space_id}/files")
}

fn space_file_path(space_id: &str, file_id: &str) -> String {
    format!("/spaces/{space_id}/files/{file_id}")
}

fn space_file_path_with_width(space_id: &str, file_id: &str, width: i64) -> String {
    format!("/spaces/{space_id}/files/{file_id}?width={width}")
}

fn space_file_path_with_skip_bin(space_id: &str, file_id: &str, skip_bin: bool) -> String {
    format!("/spaces/{space_id}/files/{file_id}?skip_bin={skip_bin}")
}

// Lists domain paths
fn space_list_views_path(space_id: &str, list_id: &str) -> String {
    format!("/spaces/{space_id}/lists/{list_id}/views")
}

fn space_list_view_objects_path(space_id: &str, list_id: &str, view_id: &str) -> String {
    format!("/spaces/{space_id}/lists/{list_id}/views/{view_id}/objects")
}

fn space_list_objects_path(space_id: &str, list_id: &str) -> String {
    format!("/spaces/{space_id}/lists/{list_id}/objects")
}

fn space_list_object_path(space_id: &str, list_id: &str, object_id: &str) -> String {
    format!("/spaces/{space_id}/lists/{list_id}/objects/{object_id}")
}

// Members domain paths
fn space_members_path(space_id: &str) -> String {
    format!("/spaces/{space_id}/members")
}

fn space_member_path(space_id: &str, member_id: &str) -> String {
    format!("/spaces/{space_id}/members/{member_id}")
}

// Spaces domain paths
fn spaces_path() -> &'static str {
    "/spaces"
}

fn space_path(space_id: &str) -> String {
    format!("/spaces/{space_id}")
}

// Auth domain paths
fn auth_challenges_path() -> &'static str {
    "/auth/challenges"
}

fn auth_api_keys_path() -> &'static str {
    "/auth/api_keys"
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
        // Properties domain (broadened centralization)
        assert_eq!(space_properties_path("s1"), "/spaces/s1/properties");
        assert_eq!(space_property_path("s1", "p1"), "/spaces/s1/properties/p1");
        // Tags domain (broadened centralization)
        assert_eq!(
            space_property_tags_path("s1", "p1"),
            "/spaces/s1/properties/p1/tags"
        );
        assert_eq!(
            space_property_tag_path("s1", "p1", "t1"),
            "/spaces/s1/properties/p1/tags/t1"
        );
        // Files domain (broadened centralization)
        assert_eq!(space_files_path("s1"), "/spaces/s1/files");
        assert_eq!(space_file_path("s1", "f1"), "/spaces/s1/files/f1");
        assert_eq!(
            space_file_path_with_width("s1", "f1", 100),
            "/spaces/s1/files/f1?width=100"
        );
        assert_eq!(
            space_file_path_with_skip_bin("s1", "f1", true),
            "/spaces/s1/files/f1?skip_bin=true"
        );
        // Lists domain (broadened centralization)
        assert_eq!(
            space_list_views_path("s1", "l1"),
            "/spaces/s1/lists/l1/views"
        );
        assert_eq!(
            space_list_view_objects_path("s1", "l1", "v1"),
            "/spaces/s1/lists/l1/views/v1/objects"
        );
        assert_eq!(
            space_list_objects_path("s1", "l1"),
            "/spaces/s1/lists/l1/objects"
        );
        assert_eq!(
            space_list_object_path("s1", "l1", "o1"),
            "/spaces/s1/lists/l1/objects/o1"
        );
        // Members domain (broadened centralization)
        assert_eq!(space_members_path("s1"), "/spaces/s1/members");
        assert_eq!(space_member_path("s1", "m1"), "/spaces/s1/members/m1");
        // Spaces domain (broadened centralization)
        assert_eq!(spaces_path(), "/spaces");
        assert_eq!(space_path("s1"), "/spaces/s1");
        // Auth domain (broadened centralization)
        assert_eq!(auth_challenges_path(), "/auth/challenges");
        assert_eq!(auth_api_keys_path(), "/auth/api_keys");
    }
}
