use crate::models::DatabaseId;
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
pub struct Formula {
    /// Formula to evaluate for this property
    expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Relation {
    /// The database this relation refers to.
    /// New linked pages must belong to this database in order to be valid.
    database_id: DatabaseId,
    /// By default, relations are formed as two synced properties across databases:
    ///     if you make a change to one property, it updates the synced property at the same time.
    /// `synced_property_name` refers to the name of the property in the related database.
    synced_property_name: Option<String>,
    /// By default, relations are formed as two synced properties across databases:
    ///     if you make a change to one property, it updates the synced property at the same time.
    /// `synced_property_id` refers to the id of the property in the related database.
    /// This is usually a short string of random letters and symbols.
    synced_property_id: Option<PropertyId>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    CountAll,
    CountValues,
    CountUniqueValues,
    CountEmpty,
    CountNotEmpty,
    PercentEmpty,
    PercentNotEmpty,
    Sum,
    Average,
    Median,
    Min,
    Max,
    Range,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Rollup {
    /// The name of the relation property this property is responsible for rolling up.
    relation_property_name: String,
    /// The id of the relation property this property is responsible for rolling up.
    relation_property_id: PropertyId,
    /// The name of the property of the pages in the related database
    /// that is used as an input to `function`.
    rollup_property_name: String,
    /// The id of the property of the pages in the related database
    /// that is used as an input to `function`.
    rollup_property_id: String,
    /// The function that is evaluated for every page in the relation of the rollup.
    function: RollupFunction,
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
    /// Represents a Checkbox Property
    /// See https://developers.notion.com/reference/database#checkbox-configuration
    Checkbox { id: PropertyId },
    /// Represents a URL Property
    /// See https://developers.notion.com/reference/database#url-configuration
    URL { id: PropertyId },
    /// Represents a Email Property
    /// See https://developers.notion.com/reference/database#email-configuration
    Email { id: PropertyId },
    /// Represents a Phone number Property
    /// See https://developers.notion.com/reference/database#phone-number-configuration
    PhoneNumber { id: PropertyId },
    /// See https://developers.notion.com/reference/database#formula-configuration
    Formula { id: PropertyId, formula: Formula },
    /// See https://developers.notion.com/reference/database#relation-configuration
    Relation { id: PropertyId, relation: Relation },
    /// See https://developers.notion.com/reference/database#rollup-configuration
    Rollup { id: PropertyId, rollup: Rollup },
    /// See https://developers.notion.com/reference/database#created-time-configuration
    CreatedTime { id: PropertyId },
    /// See https://developers.notion.com/reference/database#created-by-configuration
    CreatedBy { id: PropertyId },
    /// See https://developers.notion.com/reference/database#last-edited-time-configuration
    LastEditTime { id: PropertyId },
    /// See https://developers.notion.com/reference/database#last-edited-by-configuration
    LastEditBy { id: PropertyId },
}
