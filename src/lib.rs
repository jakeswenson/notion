use crate::models::search::{DatabaseQuery, SearchRequest};
use crate::models::{Block, BlockId, Database, DatabaseId, ListResponse, Object, Page};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client, ClientBuilder, RequestBuilder};
use serde::de::DeserializeOwned;
use snafu::{ResultExt, Snafu};

pub mod models;

const NOTION_API_VERSION: &str = "2021-05-13";

/// An wrapper Error type for all errors produced by the [`NotionApi`](NotionApi) client.
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid Notion API Token: {}", source))]
    InvalidApiToken {
        source: reqwest::header::InvalidHeaderValue,
    },
    #[snafu(display("Unable to build reqwest HTTP client: {}", source))]
    ErrorBuildingClient { source: reqwest::Error },
    #[snafu(display("Error sending HTTP request: {}", source))]
    RequestFailed { source: reqwest::Error },

    #[snafu(display("Error reading response: {}", source))]
    ResponseError { source: reqwest::Error },

    #[snafu(display("Error parsing json response: {}", source))]
    JsonParseError { source: serde_json::Error },
}

/// Meant to be a helpful trait allowing anything that can be
/// identified by the type specified in `ById`.
pub trait AsIdentifier<ById> {
    fn id(&self) -> ById;
}

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

        let mut auth_value =
            HeaderValue::from_str(&format!("Bearer {}", api_token)).context(InvalidApiToken)?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context(ErrorBuildingClient)?;

        Ok(Self { client })
    }

    async fn make_json_request<T>(request: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let json = request
            .send()
            .await
            .context(RequestFailed)?
            .text()
            .await
            .context(ResponseError)?;
        #[cfg(test)]
        {
            println!("JSON: {}", json);
            dbg!(serde_json::from_str::<serde_json::Value>(&json).context(JsonParseError)?);
        }
        let result = serde_json::from_str(&json).context(JsonParseError)?;
        Ok(result)
    }

    /// List all the databases shared with the supplied integration token.
    /// > This method is apparently deprecated/"not recommended" and
    /// > [search()](Self::search()) should be used instead.
    pub async fn list_databases(&self) -> Result<ListResponse<Database>, Error> {
        let builder = self.client.get("https://api.notion.com/v1/databases");

        Ok(NotionApi::make_json_request(builder).await?)
    }

    /// Search all pages in notion.
    /// `query` can either be a [SearchRequest] or a slightly more convenient
    /// [NotionSearch](models::search::NotionSearch) query.
    pub async fn search<T: Into<SearchRequest>>(
        &self,
        query: T,
    ) -> Result<ListResponse<Object>, Error> {
        Ok(NotionApi::make_json_request(
            self.client
                .post("https://api.notion.com/v1/search")
                .json(&query.into()),
        )
        .await?)
    }

    /// Get a database by [DatabaseId].
    pub async fn get_database<T: AsIdentifier<DatabaseId>>(
        &self,
        database_id: T,
    ) -> Result<Database, Error> {
        Ok(NotionApi::make_json_request(self.client.get(format!(
            "https://api.notion.com/v1/databases/{}",
            database_id.id().id()
        )))
        .await?)
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
        Ok(NotionApi::make_json_request(
            self.client
                .post(&format!(
                    "https://api.notion.com/v1/databases/{database_id}/query",
                    database_id = database.id()
                ))
                .json(&query.into()),
        )
        .await?)
    }

    pub async fn get_block_children<T: AsIdentifier<BlockId>>(
        &self,
        block_id: T,
    ) -> Result<ListResponse<Block>, Error> {
        Ok(NotionApi::make_json_request(self.client.get(&format!(
            "https://api.notion.com/v1/blocks/{block_id}/children",
            block_id = block_id.id()
        )))
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::search::PropertyCondition::Text;
    use crate::models::search::{
        DatabaseQuery, FilterCondition, FilterProperty, FilterValue, NotionSearch, TextCondition,
    };
    use crate::models::Object;
    use crate::NotionApi;

    fn test_token() -> String {
        let token = {
            if let Some(token) = std::env::var("NOTION_API_TOKEN").ok() {
                token
            } else if let Some(token) = std::fs::read_to_string(".api_token").ok() {
                token
            } else {
                panic!("No API Token found in environment variable 'NOTION_API_TOKEN'!")
            }
        };
        token.trim().to_string()
    }

    fn test_client() -> NotionApi {
        NotionApi::new(test_token()).unwrap()
    }

    #[tokio::test]
    async fn list_databases() -> Result<(), Box<dyn std::error::Error>> {
        let api = test_client();

        dbg!(api.list_databases().await?);

        Ok(())
    }

    #[tokio::test]
    async fn search_databases() -> Result<(), Box<dyn std::error::Error>> {
        let api = test_client();

        let response = api
            .search(NotionSearch::Filter {
                property: FilterProperty::Object,
                value: FilterValue::Database,
            })
            .await?;

        assert!(response.results.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn search_pages() -> Result<(), Box<dyn std::error::Error>> {
        let api = test_client();

        let response = api
            .search(NotionSearch::Filter {
                property: FilterProperty::Object,
                value: FilterValue::Page,
            })
            .await?;

        assert!(response.results.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn get_database() -> Result<(), Box<dyn std::error::Error>> {
        let api = test_client();

        let response = api
            .search(NotionSearch::Filter {
                value: FilterValue::Database,
                property: FilterProperty::Object,
            })
            .await?;

        let db = response
            .results()
            .iter()
            .filter_map(|o| match o {
                Object::Database { database } => Some(database),
                _ => None,
            })
            .next()
            .expect("Test expected to find at least one database in notion")
            .clone();

        // todo: fix this clone issue
        let db_result = api.get_database(db.clone()).await?;

        assert_eq!(db, db_result);

        Ok(())
    }

    #[tokio::test]
    async fn get_block_children() -> Result<(), Box<dyn std::error::Error>> {
        let api = test_client();

        let search_response = api
            .search(NotionSearch::Filter {
                value: FilterValue::Page,
                property: FilterProperty::Object,
            })
            .await?;

        println!("{:?}", search_response.results.len());

        for object in search_response.results {
            match object {
                Object::Page { page } => api.get_block_children(page).await.unwrap(),
                _ => panic!("Should not have received anything but pages!"),
            };
        }

        Ok(())
    }

    #[tokio::test]
    async fn query_database() -> Result<(), Box<dyn std::error::Error>> {
        let api = test_client();

        let response = api
            .search(NotionSearch::Filter {
                value: FilterValue::Database,
                property: FilterProperty::Object,
            })
            .await?;

        let db = response
            .results()
            .iter()
            .filter_map(|o| match o {
                Object::Database { database } => Some(database),
                _ => None,
            })
            .next()
            .expect("Test expected to find at least one database in notion")
            .clone();

        let pages = api
            .query_database(
                db,
                DatabaseQuery {
                    filter: Some(FilterCondition {
                        property: "Name".to_string(),
                        condition: Text(TextCondition::Contains("First".to_string())),
                    }),
                    ..Default::default()
                },
            )
            .await?;

        assert_eq!(pages.results().len(), 1);

        Ok(())
    }
}
