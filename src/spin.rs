use http::Response;
use serde::Serialize;
use spin_sdk::http::conversions::IntoHeaders;
use spin_sdk::http::{send, Method, Request};

use crate::models::Object;
use crate::{Error, BASE_URL, NOTION_API_VERSION};

/// An API client for Notion.
/// Create a client by using [new(api_token: String)](Self::new()).
#[derive(Clone)]
pub struct NotionApi {
    api_token: String,
}

impl NotionApi {
    /// Creates an instance of NotionApi.
    /// May fail if the provided api_token is an improper value.
    pub fn new(api_token: String) -> Result<Self, Error> {
        Ok(Self { api_token })
    }

    fn headers(&self) -> Result<impl IntoHeaders, Error> {
        let auth_value = format!("Bearer {}", self.api_token);
        // auth_value.set_sensitive(true);
        Ok(vec![
            (
                "Notion-Version".to_string(),
                NOTION_API_VERSION.as_bytes().to_vec(),
            ),
            ("Authorization".to_string(), auth_value.as_bytes().to_vec()),
        ])
    }

    async fn send_request(
        &self,
        request: Request,
    ) -> Result<Object, Error> {
        tracing::debug!("send_request");
        let response: Response<String> = send(request).await?;
        tracing::debug!(?response);
        Ok(serde_json::from_str(&response.body().to_string())?)
    }

    pub async fn get(
        &self,
        path: &str,
    ) -> Result<Object, Error> {
        let request = Request::get(format!("{BASE_URL}/{path}"))
            .headers(self.headers()?)
            .method(Method::Get)
            .build();

        self.send_request(request).await
    }

    pub async fn post(
        &self,
        path: &str,
        body: impl Serialize,
    ) -> Result<Object, Error> {
        let body = serde_json::to_vec(&body)?;
        let len = body.len();
        let request = Request::post(format!("{BASE_URL}/{path}"), body)
            .headers(self.headers()?)
            .header("Content-Type", "application/json")
            .header("Content-Length", format!("{len}"))
            .method(Method::Post)
            .build();

        self.send_request(request).await
    }
}
