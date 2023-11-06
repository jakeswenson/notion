use crate::ids::{BlockId, DatabaseId};
use crate::models::error::ErrorResponse;
use crate::models::search::{DatabaseQuery, SearchRequest};
use crate::models::{Database, ListResponse, Object, Page};
pub use chrono;
use http::header;
use ids::{AsIdentifier, PageId};
use models::block::Block;
use models::users::User;
use models::PageCreateRequest;

#[cfg(target_arch = "wasm32")]
mod spin;
#[cfg(target_arch = "wasm32")]
use spin as notion_api;

#[cfg(not(target_arch = "wasm32"))]
mod request;
#[cfg(not(target_arch = "wasm32"))]
use request as notion_api;

pub mod ids;
pub mod models;

pub use notion_api::NotionApi;

const NOTION_API_VERSION: &str = "2022-06-28";
const BASE_URL: &str = "https://api.notion.com/v1";

/// An wrapper Error type for all errors produced by the [`NotionApi`](NotionApi) client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    InvalidApiToken(#[from] header::InvalidHeaderValue),

    #[error("Error reading response: {}", source)]
    ResponseIoError { source: http::Error },

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("Unexpected API Response")]
    UnexpectedResponse { response: Object },

    #[error("API Error {}({}): {}", .error.code, .error.status, .error.message)]
    ApiError { error: ErrorResponse },
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    Send(#[from] spin_sdk::http::SendError),

    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    Http(#[from] http::Error),

    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    Url(#[from] url::ParseError),
}

impl NotionApi {
    /// List all the databases shared with the supplied integration token.
    /// > This method is apparently deprecated/"not recommended" and
    /// > [search()](Self::search()) should be used instead.
    pub async fn list_databases(&self) -> Result<ListResponse<Database>, Error> {
        match self.get("databases").await? {
            Object::List { list } => Ok(list.expect_databases()?),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Search all pages in notion.
    /// `query` can either be a [SearchRequest] or a slightly more convenient
    /// [NotionSearch](models::search::NotionSearch) query.
    pub async fn search<T: Into<SearchRequest>>(
        &self,
        query: T,
    ) -> Result<ListResponse<Object>, Error> {
        let search_request: SearchRequest = query.into();
        let search_request = serde_json::to_vec(&search_request)?;
        match self.post("search", search_request).await? {
            Object::List { list } => Ok(list),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Get a database by [DatabaseId].
    pub async fn get_database<T: AsIdentifier<DatabaseId>>(
        &self,
        database_id: T,
    ) -> Result<Database, Error> {
        match self
            .get(&format!("databases/{}", database_id.as_id()))
            .await?
        {
            Object::Database { database } => Ok(database),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Get a page by [PageId].
    pub async fn get_page<T: AsIdentifier<PageId>>(
        &self,
        page_id: T,
    ) -> Result<Page, Error> {
        match self.get(&format!("pages/{}", page_id.as_id())).await? {
            Object::Page { page } => Ok(page),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Creates a new page and return the created page
    pub async fn create_page<T: Into<PageCreateRequest>>(
        &self,
        page: T,
    ) -> Result<Page, Error> {
        let page: PageCreateRequest = page.into();
        let page = serde_json::to_vec(&page)?;
        match self.post("pages", page).await? {
            Object::Page { page } => Ok(page),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    /// Query a database and return the matching pages.
    pub async fn query_database<D, T>(
        &self,
        database: D,
        query: T,
    ) -> Result<ListResponse<Page>, Error>
    where
        T: Into<DatabaseQuery>,
        D: AsIdentifier<DatabaseId>,
    {
        let query: DatabaseQuery = query.into();
        match self
            .post(
                &format!(
                    "databases/{database_id}/query",
                    database_id = database.as_id()
                ),
                query,
            )
            .await?
        {
            Object::List { list } => Ok(list.expect_pages()?),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    pub async fn get_block_children<T: AsIdentifier<BlockId>>(
        &self,
        block_id: T,
    ) -> Result<ListResponse<Block>, Error> {
        match self
            .get(&format!(
                "blocks/{block_id}/children",
                block_id = block_id.as_id()
            ))
            .await?
        {
            Object::List { list } => Ok(list.expect_blocks()?),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }

    pub async fn list_users(&self) -> Result<ListResponse<User>, Error> {
        match self.get("users").await? {
            Object::List { list } => Ok(list.expect_users()?),
            response => Err(Error::UnexpectedResponse { response }),
        }
    }
}
