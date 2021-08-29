use crate::ids::BlockId;
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
            Object::Page { page } => api
                .get_block_children(BlockId::from(page.id))
                .await
                .unwrap(),
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
