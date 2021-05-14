use crate::models::search::SearchRequest;
use crate::models::{Database, ListResponse};
use std::collections::HashMap;

mod models;

struct NotionApi {
    token: String,
}

impl NotionApi {
    /// This method is apparently deprecated
    pub async fn list_databases(
        &self,
    ) -> Result<ListResponse<Database>, Box<dyn std::error::Error>> {
        let client = reqwest::ClientBuilder::new().build()?;
        let json = client
            .get("https://api.notion.com/v1/databases")
            .bearer_auth(self.token.clone())
            .send()
            .await?
            .text()
            .await?;
        dbg!(&json);
        let result = serde_json::from_str(&json)?;

        Ok(result)
    }

    pub async fn search<T: Into<SearchRequest>>(
        &self,
        query: T,
    ) -> Result<ListResponse<Database>, Box<dyn std::error::Error>> {
        let client = reqwest::ClientBuilder::new().build()?;
        let json = client
            .post("https://api.notion.com/v1/search")
            .bearer_auth(self.token.clone())
            .json(&query.into())
            .send()
            .await?
            .text()
            .await?;

        dbg!(serde_json::from_str::<serde_json::Value>(&json)?);
        let result = serde_json::from_str(&json)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::search::{FilterProperty, FilterValue, NotionSearch};
    use crate::NotionApi;
    const TEST_TOKEN: &'static str = include_str!(".api_token");

    #[tokio::test]
    async fn list_databases() -> Result<(), Box<dyn std::error::Error>> {
        let api = NotionApi {
            token: TEST_TOKEN.to_string(),
        };

        dbg!(api.list_databases().await?);

        Ok(())
    }

    #[tokio::test]
    async fn search() -> Result<(), Box<dyn std::error::Error>> {
        let api = NotionApi {
            token: TEST_TOKEN.to_string(),
        };

        dbg!(
            api.search(NotionSearch::Filter {
                value: FilterValue::Database,
                property: FilterProperty::Object
            })
            .await?
        );

        Ok(())
    }
}
