pub mod paging;
pub mod properties;
pub mod search;
pub mod text;

use crate::models::properties::PropertyConfiguration;
use crate::models::text::RichText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
enum ObjectType {
    Database,
    List,
}

/// A zero-cost wrapper type around a Database ID
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct DatabaseId(String);

/// Represents a Notion Database
/// See https://developers.notion.com/reference/database
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Database {
    /// Always "database"
    object: ObjectType,
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct ListResponse<T> {
    object: ObjectType,
    results: Vec<T>,
    next_cursor: Option<String>,
    has_more: bool,
}
