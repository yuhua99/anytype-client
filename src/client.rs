use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use reqwest::{Client as HttpClient, Method, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use url::Url;

use crate::{ANYTYPE_VERSION, models::*};

const PAGE_LIMIT: i64 = 1000;

pub struct AnytypeClient {
    http: HttpClient,
    base_url: Url,
    api_key: Option<String>,
}

impl AnytypeClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Result<Self> {
        Ok(Self {
            http: HttpClient::builder()
                .timeout(Duration::from_secs(30))
                .build()?,
            base_url: Url::parse(&base_url)
                .with_context(|| format!("invalid base URL: {base_url}"))?,
            api_key,
        })
    }

    async fn request<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = self.url(path)?;
        let mut req = self
            .http
            .request(method, url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Anytype-Version", ANYTYPE_VERSION);

        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        if let Some(body) = body {
            req = req.json(body);
        }

        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(anyhow!("request failed with status {status}: {text}"));
        }
        if status == StatusCode::NO_CONTENT || text.trim().is_empty() {
            return serde_json::from_str("null").map_err(Into::into);
        }
        serde_json::from_str(&text).with_context(|| format!("failed to decode response: {text}"))
    }

    async fn request_empty<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<()> {
        let _: Value = self.request(method, path, body).await?;
        Ok(())
    }

    async fn request_paginated<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<DataResponse<T>> {
        let mut offset = 0;
        let mut data = Vec::new();

        loop {
            let paged_path = format!("{path}?offset={offset}&limit={PAGE_LIMIT}");
            let mut response: DataResponse<T> =
                self.request(method.clone(), &paged_path, body).await?;
            let page_len = response.data.len();
            let has_more = response
                .pagination
                .as_ref()
                .and_then(|pagination| pagination.has_more)
                .unwrap_or(false);
            let pagination = response.pagination.take();

            data.append(&mut response.data);

            if !has_more {
                return Ok(DataResponse { data, pagination });
            }
            if page_len == 0 {
                return Err(anyhow!(
                    "pagination stalled for {path}: has_more=true but page was empty"
                ));
            }
            offset += PAGE_LIMIT;
        }
    }

    fn url(&self, api_path: &str) -> Result<Url> {
        let mut url = self.base_url.clone();
        let base_path = url.path().trim_end_matches('/');
        let (api_path, query) = api_path
            .trim_start_matches('/')
            .split_once('?')
            .map_or((api_path.trim_start_matches('/'), None), |(path, query)| {
                (path, Some(query))
            });
        url.set_path(&format!("{base_path}/v1/{api_path}"));
        url.set_query(query);
        Ok(url)
    }

    pub async fn create_challenge(&self, app_name: &str) -> Result<CreateChallengeResponse> {
        self.request(
            Method::POST,
            "/auth/challenges",
            Some(&serde_json::json!({ "app_name": app_name })),
        )
        .await
    }

    pub async fn create_api_key(
        &self,
        challenge_id: &str,
        code: &str,
    ) -> Result<CreateApiKeyResponse> {
        self.request(
            Method::POST,
            "/auth/api_keys",
            Some(&serde_json::json!({ "challenge_id": challenge_id, "code": code })),
        )
        .await
    }

    pub async fn spaces(&self) -> Result<SpaceListResponse> {
        self.request_paginated(Method::GET, "/spaces", Option::<&()>::None)
            .await
    }

    pub async fn create_space(&self, req: &CreateSpaceRequest) -> Result<CreateSpaceResponse> {
        self.request(Method::POST, "/spaces", Some(req)).await
    }

    pub async fn space(&self, id: &str) -> Result<SpaceResponse> {
        self.request(Method::GET, &format!("/spaces/{id}"), Option::<&()>::None)
            .await
    }

    pub async fn objects(&self, space_id: &str) -> Result<DataResponse<Object>> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/objects"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn create_object(
        &self,
        space_id: &str,
        req: &CreateObjectRequest,
    ) -> Result<ObjectResponse> {
        self.request(
            Method::POST,
            &format!("/spaces/{space_id}/objects"),
            Some(req),
        )
        .await
    }

    pub async fn object(&self, space_id: &str, object_id: &str) -> Result<ObjectResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/objects/{object_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn delete_object(&self, space_id: &str, object_id: &str) -> Result<ObjectResponse> {
        self.request(
            Method::DELETE,
            &format!("/spaces/{space_id}/objects/{object_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn search(&self, req: &SearchRequest) -> Result<SearchResponse> {
        self.request_paginated(Method::POST, "/search", Some(req))
            .await
    }

    pub async fn space_search(
        &self,
        space_id: &str,
        req: &SearchRequest,
    ) -> Result<SearchResponse> {
        self.request_paginated(
            Method::POST,
            &format!("/spaces/{space_id}/search"),
            Some(req),
        )
        .await
    }

    pub async fn types(&self, space_id: &str) -> Result<DataResponse<ObjectType>> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/types"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn type_get(&self, space_id: &str, type_id: &str) -> Result<TypeResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/types/{type_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn templates(&self, space_id: &str, type_id: &str) -> Result<DataResponse<Template>> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/types/{type_id}/templates"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn template(
        &self,
        space_id: &str,
        type_id: &str,
        template_id: &str,
    ) -> Result<TemplateResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/types/{type_id}/templates/{template_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn views(&self, space_id: &str, list_id: &str) -> Result<ViewListResponse> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/lists/{list_id}/views"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn view_objects(
        &self,
        space_id: &str,
        list_id: &str,
        view_id: &str,
    ) -> Result<ObjectListResponse> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/lists/{list_id}/views/{view_id}/objects"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn add_to_list(
        &self,
        space_id: &str,
        list_id: &str,
        objects: &[String],
    ) -> Result<()> {
        self.request_empty(
            Method::POST,
            &format!("/spaces/{space_id}/lists/{list_id}/objects"),
            Some(&serde_json::json!({ "objects": objects })),
        )
        .await
    }

    pub async fn remove_from_list(
        &self,
        space_id: &str,
        list_id: &str,
        object_id: &str,
    ) -> Result<()> {
        self.request_empty(
            Method::DELETE,
            &format!("/spaces/{space_id}/lists/{list_id}/objects/{object_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn members(&self, space_id: &str) -> Result<MemberListResponse> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/members"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn member(&self, space_id: &str, member_id: &str) -> Result<MemberResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/members/{member_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn resolve_space(&self, id_or_name: &str) -> Result<String> {
        let spaces = self.spaces().await?.data;
        if spaces.iter().any(|space| space.id == id_or_name) {
            return Ok(id_or_name.to_string());
        }
        if let Some(space) = spaces
            .iter()
            .find(|space| space.name.eq_ignore_ascii_case(id_or_name))
        {
            return Ok(space.id.clone());
        }
        let needle = id_or_name.to_lowercase();
        let matches: Vec<_> = spaces
            .iter()
            .filter(|space| space.name.to_lowercase().contains(&needle))
            .collect();
        match matches.len() {
            0 => Ok(id_or_name.to_string()),
            1 => Ok(matches[0].id.clone()),
            _ => Err(anyhow!(
                "space not found: multiple spaces matched '{}': {}",
                id_or_name,
                matches
                    .iter()
                    .map(|s| format!("{} ({})", s.name, s.id))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
}
