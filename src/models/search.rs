use crate::models::paging::Paging;
use serde::{Deserialize, Serialize, Serializer};

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
    timestamp: SortTimestamp,
    direction: SortDirection,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Filter {
    property: FilterProperty,
    value: FilterValue,
}

#[derive(Serialize, Debug, Eq, PartialEq, Default)]
pub struct SearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,
    #[serde(flatten)]
    paging: Option<Paging>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextCondition {
    Equals(String),
    DoesNotEqual(String),
    Contains(String),
    DoesNotContain(String),
    StartsWith(String),
    EndsWith(String),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

fn serialize_to_true<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(true)
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PropertyCondition {
    Text(TextCondition),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FilterCondition {
    pub property: String,
    #[serde(flatten)]
    pub condition: PropertyCondition,
}

#[derive(Serialize, Debug, Eq, PartialEq, Default)]
pub struct DatabaseQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterCondition>,
    #[serde(flatten)]
    pub paging: Option<Paging>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NotionSearch {
    Query(String),
    Sort {
        timestamp: SortTimestamp,
        direction: SortDirection,
    },
    Filter {
        property: FilterProperty,
        value: FilterValue,
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

#[cfg(test)]
mod tests {
    mod text_filters {
        use crate::models::search::PropertyCondition::Text;
        use crate::models::search::{FilterCondition, PropertyCondition, TextCondition};

        #[test]
        fn text_property_equals() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_string(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::Equals("Test".to_string())),
            })?;
            assert_eq!(
                dbg!(json),
                r#"{"property":"Name","text":{"equals":"Test"}}"#
            );

            Ok(())
        }

        #[test]
        fn text_property_contains() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_string(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::Contains("Test".to_string())),
            })?;
            assert_eq!(
                dbg!(json),
                r#"{"property":"Name","text":{"contains":"Test"}}"#
            );

            Ok(())
        }

        #[test]
        fn text_property_is_empty() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_string(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::IsEmpty),
            })?;
            assert_eq!(
                dbg!(json),
                r#"{"property":"Name","text":{"is_empty":true}}"#
            );

            Ok(())
        }

        #[test]
        fn text_property_is_not_empty() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_string(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::IsNotEmpty),
            })?;
            assert_eq!(
                dbg!(json),
                r#"{"property":"Name","text":{"is_not_empty":true}}"#
            );

            Ok(())
        }
    }
}
