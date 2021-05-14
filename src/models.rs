pub mod search;

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

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
struct PropertyId(String);

/// How the number is displayed in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum NumberFormat {
    Number,
    NumberWithCommas,
    Percent,
    Dollar,
    Euro,
    Pound,
    Yen,
    Ruble,
    Rupee,
    Won,
    Yuan,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct SelectOptionId(String);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SelectOption {
    name: String,
    id: SelectOptionId,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Select {
    /// Sorted list of options available for this property.
    options: Vec<SelectOption>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PropertyConfiguration {
    /// Represents the special Title property required on every database.
    /// See https://developers.notion.com/reference/database#title-configuration
    Title { id: PropertyId },
    /// Represents a Text property
    /// https://developers.notion.com/reference/database#text-configuration
    #[serde(rename = "rich_text")]
    Text { id: PropertyId },
    /// Represents a Number Property
    /// See https://developers.notion.com/reference/database#number-configuration
    Number {
        id: PropertyId,
        /// How the number is displayed in Notion.
        format: NumberFormat,
    },
    /// Represents a Select Property
    /// See https://developers.notion.com/reference/database#select-configuration
    Select { id: PropertyId, select: Select },
    /// Represents a Date Property
    /// See https://developers.notion.com/reference/database#date-configuration
    Date { id: PropertyId },
    /// Represents a File Property
    /// See https://developers.notion.com/reference/database#date-configuration
    File { id: PropertyId },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextColor {
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

/// Rich text annotations
/// See https://developers.notion.com/reference/rich-text#annotations
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Annotations {
    bold: bool,
    code: bool,
    color: TextColor,
    italic: bool,
    strikethrough: bool,
    underline: bool,
}

/// Properties common on all rich text objects
/// See https://developers.notion.com/reference/rich-text#all-rich-text
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RichTextCommon {
    plain_text: String,
    href: Option<String>,
    annotations: Annotations,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Link {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Text {
    content: String,
    link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RichText {
    /// See https://developers.notion.com/reference/rich-text#text-objects
    Text {
        #[serde(flatten)]
        rich_text: RichTextCommon,
        text: Text,
    },
    /// See https://developers.notion.com/reference/rich-text#mention-objects
    Mention {
        #[serde(flatten)]
        rich_text: RichTextCommon,
    },
    /// See https://developers.notion.com/reference/rich-text#equation-objects
    Equation {
        #[serde(flatten)]
        rich_text: RichTextCommon,
    },
}
