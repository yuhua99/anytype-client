use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use reqwest::{Client as HttpClient, Method, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use url::Url;

use crate::{ANYTYPE_VERSION, models::DataResponse};

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

    pub(super) async fn request<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = self.url(path);
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

    pub(super) async fn request_empty<B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<()> {
        let _: Value = self.request(method, path, body).await?;
        Ok(())
    }

    pub(super) async fn request_paginated<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<DataResponse<T>> {
        let mut offset = 0;
        let mut data = Vec::new();

        loop {
            let paged_path = paginated_path(path, offset, PAGE_LIMIT);
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

    fn url(&self, api_path: &str) -> Url {
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
        url
    }
}

fn paginated_path(path: &str, offset: i64, limit: i64) -> String {
    let separator = if path.contains('?') { '&' } else { '?' };
    format!("{path}{separator}offset={offset}&limit={limit}")
}
