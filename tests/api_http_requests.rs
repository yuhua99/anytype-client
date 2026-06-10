use anyclient::api::{AnytypeClient, PageOptions};
use anyclient::models::SearchRequest;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ANYTYPE_VERSION: &str = "2025-11-08";

fn minimal_object(id: &str, name: &str, space_id: &str) -> serde_json::Value {
    json!({
        "id": id,
        "name": name,
        "space_id": space_id,
        "layout": "basic"
    })
}

#[tokio::test]
async fn objects_page_sends_correct_request_and_deserializes() {
    let server = MockServer::start().await;
    let space_id = "spc_123";
    let api_key = "test-api-key-abc";

    let expected_path = format!("/v1/spaces/{space_id}/objects");

    Mock::given(method("GET"))
        .and(path(&expected_path))
        .and(query_param("offset", "0"))
        .and(query_param("limit", "5"))
        .and(header("Anytype-Version", ANYTYPE_VERSION))
        .and(header("Accept", "application/json"))
        .and(header("Authorization", format!("Bearer {}", api_key)))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [minimal_object("obj1", "Test Object", space_id)],
            "pagination": { "has_more": false }
        })))
        .mount(&server)
        .await;

    let client = AnytypeClient::new(server.uri(), Some(api_key.to_string())).unwrap();
    let resp = client
        .objects_page(
            space_id,
            Some(PageOptions {
                offset: 0,
                limit: 5,
            }),
        )
        .await
        .unwrap();

    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].id, "obj1");
    assert_eq!(resp.data[0].name, "Test Object");
}

#[tokio::test]
async fn auto_pagination_accumulates_all_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/spaces/s1/objects"))
        .and(query_param("offset", "0"))
        .and(query_param("limit", "1000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [minimal_object("obj1", "One", "s1")],
            "pagination": { "total": 2, "has_more": true }
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/spaces/s1/objects"))
        .and(query_param("offset", "1000"))
        .and(query_param("limit", "1000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [minimal_object("obj2", "Two", "s1")],
            "pagination": { "total": 2, "has_more": false }
        })))
        .mount(&server)
        .await;

    let client = AnytypeClient::new(server.uri(), None).unwrap();
    let resp = client.objects("s1").await.unwrap();

    assert_eq!(
        resp.data.iter().map(|o| o.id.as_str()).collect::<Vec<_>>(),
        ["obj1", "obj2"]
    );
    let pagination = resp.pagination.unwrap();
    assert_eq!(pagination.total, Some(2));
    assert_eq!(pagination.has_more, Some(false));
}

#[tokio::test]
async fn auto_pagination_errors_on_stalled_pages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/spaces/s1/objects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "pagination": { "has_more": true }
        })))
        .mount(&server)
        .await;

    let client = AnytypeClient::new(server.uri(), None).unwrap();
    let err = client.objects("s1").await.unwrap_err().to_string();

    assert!(err.contains("pagination stalled"), "{err}");
}

#[tokio::test]
async fn http_error_includes_method_and_path() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/spaces/spc_404/objects/obj_404"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"error":"not found"})))
        .mount(&server)
        .await;

    let client = AnytypeClient::new(server.uri(), Some("key".to_string())).unwrap();
    let err = client
        .object("spc_404", "obj_404", None)
        .await
        .unwrap_err()
        .to_string();

    assert!(err.contains("GET /spaces/spc_404/objects/obj_404"), "{err}");
    assert!(err.contains("404"), "{err}");
    assert!(err.contains("not found"), "{err}");
}

#[tokio::test]
async fn search_page_sends_typed_body_and_deserializes() {
    let server = MockServer::start().await;
    let api_key = "test-api-key-abc";

    let search_req = SearchRequest {
        query: "hello world".into(),
        types: vec!["page".into()],
        filters: None,
        sort: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/search"))
        .and(query_param("offset", "0"))
        .and(query_param("limit", "5"))
        .and(header("Anytype-Version", ANYTYPE_VERSION))
        .and(header("Accept", "application/json"))
        .and(header("Authorization", format!("Bearer {}", api_key)))
        .and(body_json(serde_json::to_value(&search_req).unwrap()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [minimal_object("res1", "Search Result", "spc_x")],
            "pagination": { "has_more": false }
        })))
        .mount(&server)
        .await;

    let client = AnytypeClient::new(server.uri(), Some(api_key.to_string())).unwrap();
    let resp: anyclient::models::SearchResponse = client
        .search_page(
            &search_req,
            Some(PageOptions {
                offset: 0,
                limit: 5,
            }),
        )
        .await
        .unwrap();

    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].id, "res1");
}
