use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct PropertyId(String);

/// How the number is displayed in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NumberFormat {
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
    /// See https://developers.notion.com/reference/database#file-configuration
    /// Documentation issue: docs claim type name is `file` but it's is in fact `files`
    Files { id: PropertyId },
}
