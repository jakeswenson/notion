use crate::models::paging::Paging;
use crate::models::{Number, PageId, UserId};
use chrono::{DateTime, Utc};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortTimestamp {
    LastEditedTime,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterValue {
    Page,
    Database,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilterProperty {
    Object,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Sort {
    /// The name of the timestamp to sort against.
    timestamp: SortTimestamp,
    direction: SortDirection,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
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

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
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

fn serialize_to_empty_object<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Todo: there has to be a better way?
    serializer.serialize_map(Some(0))?.end()
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NumberCondition {
    Equals(Number),
    DoesNotEqual(Number),
    GreaterThan(Number),
    LessThan(Number),
    GreaterThanOrEqualTo(Number),
    LessThanOrEqualTo(Number),
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CheckboxCondition {
    Equals(bool),
    DoesNotEqual(bool),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SelectCondition {
    /// Only return pages where the page property value matches the provided value exactly.
    Equals(String),
    /// Only return pages where the page property value does not match the provided value exactly.
    DoesNotEqual(String),
    /// Only return pages where the page property value is empty.
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    /// Only return pages where the page property value is present.
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MultiSelectCondition {
    /// Only return pages where the page property value contains the provided value.
    Contains(String),
    /// Only return pages where the page property value does not contain the provided value.
    DoesNotContain(String),
    /// Only return pages where the page property value is empty.
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    /// Only return pages where the page property value is present.
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DateCondition {
    /// Only return pages where the page property value matches the provided date exactly.
    /// Note that the comparison is done against the date.
    /// Any time information sent will be ignored.
    Equals(DateTime<Utc>),
    /// Only return pages where the page property value is before the provided date.
    /// Note that the comparison is done against the date.
    /// Any time information sent will be ignored.
    Before(DateTime<Utc>),
    /// Only return pages where the page property value is after the provided date.
    /// Note that the comparison is done against the date.
    /// Any time information sent will be ignored.
    After(DateTime<Utc>),
    /// Only return pages where the page property value is on or before the provided date.
    /// Note that the comparison is done against the date.
    /// Any time information sent will be ignored.
    OnOrBefore(DateTime<Utc>),
    /// Only return pages where the page property value is on or after the provided date.
    /// Note that the comparison is done against the date.
    /// Any time information sent will be ignored.
    OnOrAfter(DateTime<Utc>),
    /// Only return pages where the page property value is empty.
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    /// Only return pages where the page property value is present.
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
    /// Only return pages where the page property value is within the past week.
    #[serde(serialize_with = "serialize_to_empty_object")]
    PastWeek,
    /// Only return pages where the page property value is within the past month.
    #[serde(serialize_with = "serialize_to_empty_object")]
    PastMonth,
    /// Only return pages where the page property value is within the past year.
    #[serde(serialize_with = "serialize_to_empty_object")]
    PastYear,
    /// Only return pages where the page property value is within the next week.
    #[serde(serialize_with = "serialize_to_empty_object")]
    NextWeek,
    /// Only return pages where the page property value is within the next month.
    #[serde(serialize_with = "serialize_to_empty_object")]
    NextMonth,
    /// Only return pages where the page property value is within the next year.
    #[serde(serialize_with = "serialize_to_empty_object")]
    NextYear,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PeopleCondition {
    Contains(UserId),
    /// Only return pages where the page property value does not contain the provided value.
    DoesNotContain(UserId),
    /// Only return pages where the page property value is empty.
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    /// Only return pages where the page property value is present.
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FilesCondition {
    /// Only return pages where the page property value is empty.
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    /// Only return pages where the page property value is present.
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RelationCondition {
    /// Only return pages where the page property value contains the provided value.
    Contains(PageId),
    /// Only return pages where the page property value does not contain the provided value.
    DoesNotContain(PageId),
    /// Only return pages where the page property value is empty.
    #[serde(serialize_with = "serialize_to_true")]
    IsEmpty,
    /// Only return pages where the page property value is present.
    #[serde(serialize_with = "serialize_to_true")]
    IsNotEmpty,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FormulaCondition {
    /// Only return pages where the result type of the page property formula is "text"
    /// and the provided text filter condition matches the formula's value.
    Text(TextCondition),
    /// Only return pages where the result type of the page property formula is "number"
    /// and the provided number filter condition matches the formula's value.
    Number(NumberCondition),
    /// Only return pages where the result type of the page property formula is "checkbox"
    /// and the provided checkbox filter condition matches the formula's value.
    Checkbox(CheckboxCondition),
    /// Only return pages where the result type of the page property formula is "date"
    /// and the provided date filter condition matches the formula's value.
    Date(DateCondition),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PropertyCondition {
    Text(TextCondition),
    Number(NumberCondition),
    Checkbox(CheckboxCondition),
    Select(SelectCondition),
    MultiSelect(MultiSelectCondition),
    Date(DateCondition),
    People(PeopleCondition),
    Files(FilesCondition),
    Relation(RelationCondition),
    Formula(FormulaCondition),
    /// Returns pages when **any** of the filters inside the provided vector match.
    Or(Vec<PropertyCondition>),
    /// Returns pages when **all** of the filters inside the provided vector match.
    And(Vec<PropertyCondition>),
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct FilterCondition {
    pub property: String,
    #[serde(flatten)]
    pub condition: PropertyCondition,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseSortTimestamp {
    CreatedTime,
    LastEditedTime,
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct DatabaseSort {
    // Todo: Should property and timestamp be mutually exclusive? (i.e a flattened enum?)
    //  the documentation is not clear:
    //  https://developers.notion.com/reference/post-database-query#post-database-query-sort
    pub property: Option<String>,
    /// The name of the timestamp to sort against.
    pub timestamp: Option<DatabaseSortTimestamp>,
    pub direction: SortDirection,
}

#[derive(Serialize, Debug, Eq, PartialEq, Default)]
pub struct DatabaseQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<DatabaseSort>>,
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

impl NotionSearch {
    pub fn filter_by_databases() -> Self {
        Self::Filter {
            property: FilterProperty::Object,
            value: FilterValue::Database,
        }
    }
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
                    timestamp,
                    direction,
                }),
                ..Default::default()
            },
            NotionSearch::Filter { property, value } => SearchRequest {
                filter: Some(Filter { property, value }),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    mod text_filters {
        use crate::models::search::PropertyCondition::Text;
        use crate::models::search::{FilterCondition, TextCondition};
        use serde_json::json;

        #[test]
        fn text_property_equals() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_value(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::Equals("Test".to_string())),
            })?;
            assert_eq!(json, json!({"property":"Name","text":{"equals":"Test"}}));

            Ok(())
        }

        #[test]
        fn text_property_contains() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_value(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::Contains("Test".to_string())),
            })?;
            assert_eq!(
                dbg!(json),
                json!({"property":"Name","text":{"contains":"Test"}})
            );

            Ok(())
        }

        #[test]
        fn text_property_is_empty() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_value(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::IsEmpty),
            })?;
            assert_eq!(
                dbg!(json),
                json!({"property":"Name","text":{"is_empty":true}})
            );

            Ok(())
        }

        #[test]
        fn text_property_is_not_empty() -> Result<(), Box<dyn std::error::Error>> {
            let json = serde_json::to_value(&FilterCondition {
                property: "Name".to_string(),
                condition: Text(TextCondition::IsNotEmpty),
            })?;
            assert_eq!(
                dbg!(json),
                json!({"property":"Name","text":{"is_not_empty":true}})
            );

            Ok(())
        }
    }
}
