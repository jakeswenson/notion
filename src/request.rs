use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, ClientBuilder, Request};
use serde::Serialize;
use tracing::Instrument;

use crate::models::Object;
use crate::{Error, BASE_URL, NOTION_API_VERSION};

/// An API client for Notion.
/// Create a client by using [new(api_token: String)](Self::new()).
#[derive(Clone)]
pub struct NotionApi {
    client: Client,
}

impl NotionApi {
    /// Creates an instance of NotionApi.
    /// May fail if the provided api_token is an improper value.
    pub fn new(api_token: String) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Notion-Version",
            HeaderValue::from_static(NOTION_API_VERSION),
        );

        let mut auth_value = HeaderValue::from_str(&format!("Bearer {}", api_token))?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(Self { client })
    }

    async fn make_json_request(
        &self,
        request: Request,
    ) -> Result<Object, Error> {
        let url = request.url();
        tracing::trace!(
            method = request.method().as_str(),
            url = url.as_str(),
            "Sending request"
        );
        let json = self
            .client
            .execute(request)
            .instrument(tracing::trace_span!("Sending request"))
            .await?
            .text()
            .instrument(tracing::trace_span!("Reading response"))
            .await?;

        match serde_json::from_str(&json)? {
            Object::Error { error } => Err(Error::ApiError { error }),
            response => Ok(response),
        }
    }

    pub async fn get(
        &self,
        path: &str,
    ) -> Result<Object, Error> {
        let request = self.client.get(format!("{BASE_URL}/{path}")).build()?;
        self.make_json_request(request).await
    }

    pub async fn post(
        &self,
        path: &str,
        body: impl Serialize,
    ) -> Result<Object, Error> {
        let request = self
            .client
            .post(format!("{BASE_URL}/{path}"))
            .json(&body)
            .build()?;
        self.make_json_request(request).await
    }
}
