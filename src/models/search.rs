use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortTimestamp {
    LastEditedTime,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterValue {
    Page,
    Database,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterProperty {
    Object,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Sort {
    direction: SortDirection,
    timestamp: SortTimestamp,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Filter {
    value: FilterValue,
    property: FilterProperty,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct PagingCursor(String);

#[derive(Serialize, Debug, Eq, PartialEq, Default)]
pub struct SearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<PagingCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NotionSearch {
    Query(String),
    Sort {
        direction: SortDirection,
        timestamp: SortTimestamp,
    },
    Filter {
        value: FilterValue,
        property: FilterProperty,
    },
}

impl From<NotionSearch> for SearchRequest {
    fn from(search: NotionSearch) -> Self {
        match search {
            NotionSearch::Query(query) => SearchRequest {
                query: Some(query),
                ..Default::default()
            },
            NotionSearch::Sort {
                direction,
                timestamp,
            } => SearchRequest {
                sort: Some(Sort {
                    direction,
                    timestamp,
                }),
                ..Default::default()
            },
            NotionSearch::Filter { value, property } => SearchRequest {
                filter: Some(Filter { value, property }),
                ..Default::default()
            },
        }
    }
}
