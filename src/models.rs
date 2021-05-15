pub mod paging;
pub mod properties;
pub mod search;
pub mod text;

use crate::models::properties::{PropertyConfiguration, PropertyValue};
use crate::models::text::RichText;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Identifiable;
pub use chrono::{DateTime, Utc};
pub use serde_json::value::Number;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
#[serde(rename_all = "lowercase")]
enum ObjectType {
    Database,
    List,
}

/// A zero-cost wrapper type around a Database ID
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct DatabaseId(String);

impl DatabaseId {
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl Identifiable for DatabaseId {
    type Type = DatabaseId;

    fn id(&self) -> &Self::Type {
        self
    }
}

impl Display for DatabaseId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Represents a Notion Database
/// See https://developers.notion.com/reference/database
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Database {
    /// Unique identifier for the database.
    id: DatabaseId,
    /// Date and time when this database was created.
    created_time: DateTime<Utc>,
    /// Date and time when this database was updated.
    last_edited_time: DateTime<Utc>,
    /// Name of the database as it appears in Notion.
    title: Vec<RichText>,
    /// Schema of properties for the database as they appear in Notion.
    //
    // key string
    // The name of the property as it appears in Notion.
    //
    // value object
    // A Property object.
    properties: HashMap<String, PropertyConfiguration>,
}

impl Identifiable for Database {
    type Type = DatabaseId;

    fn id(&self) -> &Self::Type {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ListResponse<T> {
    results: Vec<T>,
    next_cursor: Option<String>,
    has_more: bool,
}

impl<T> ListResponse<T> {
    pub fn results(&self) -> &[T] {
        &self.results
    }
}

/// A zero-cost wrapper type around a Page ID
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct PageId(String);

impl PageId {
    pub fn id(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Parent {
    #[serde(rename = "database_id")]
    Database(#[serde(rename = "database_id")] DatabaseId),
    #[serde(rename = "page_id")]
    Page(#[serde(rename = "page_id")] PageId),
    Workspace,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Properties {
    #[serde(flatten)]
    properties: HashMap<String, PropertyValue>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    id: PageId,
    /// Date and time when this page was created.
    created_time: DateTime<Utc>,
    /// Date and time when this page was updated.
    last_edited_time: DateTime<Utc>,
    /// The archived status of the page.
    archived: bool,
    properties: Properties,
    parent: Parent,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Block {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "object")]
pub enum Object {
    Database {
        #[serde(flatten)]
        database: Database,
    },
    Page {},
    List {
        list: ListResponse<Object>,
    },
}

/// A zero-cost wrapper type around a Page ID
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct UserId(String);

impl UserId {
    pub fn id(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    id: UserId,
    name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum User {
    Person {
        #[serde(flatten)]
        common: UserCommon,
        person: Person,
    },
    Bot {
        #[serde(flatten)]
        common: UserCommon,
        bot: Bot,
    },
}

#[cfg(test)]
mod tests {
    use crate::models::Page;

    #[test]
    fn deserialize_page() {
        let _page: Page = serde_json::from_str(include_str!("models/tests/page.json")).unwrap();
    }
}
