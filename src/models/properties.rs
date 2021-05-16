use crate::models::text::RichText;
use crate::models::{DatabaseId, PageId, User};
use serde::{Deserialize, Serialize};

use super::{DateTime, Number, Utc};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone)]
#[serde(transparent)]
pub struct PropertyId(String);

/// How the number is displayed in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct SelectOptionId(String);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectOption {
    name: String,
    id: SelectOptionId,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Select {
    /// Sorted list of options available for this property.
    options: Vec<SelectOption>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Formula {
    /// Formula to evaluate for this property
    expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
    /// Represents a Multi-select Property
    /// See https://developers.notion.com/reference/database#multi-select-configuration
    MultiSelect {
        id: PropertyId,
        multi_select: Select,
    },
    /// Represents a Date Property
    /// See https://developers.notion.com/reference/database#date-configuration
    Date { id: PropertyId },
    /// Represents a People Property
    /// See https://developers.notion.com/reference/database#people-configuration
    People { id: PropertyId },
    /// Represents a File Property
    /// See https://developers.notion.com/reference/database#file-configuration
    // Todo: File a bug with notion
    //       Documentation issue: docs claim type name is `file` but it is in fact `files`
    Files { id: PropertyId },
    /// Represents a Checkbox Property
    /// See https://developers.notion.com/reference/database#checkbox-configuration
    Checkbox { id: PropertyId },
    /// Represents a URL Property
    /// See https://developers.notion.com/reference/database#url-configuration
    Url { id: PropertyId },
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectedValue {
    id: SelectOptionId,
    name: String,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DateValue {
    // Todo: Will this work with dates (without time)?
    //       does there need to be an enum of Date|DateTime?
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

/// Formula property value objects represent the result of evaluating a formula
/// described in the database's properties.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FormulaResultValue {
    String(#[serde(rename = "string")] Option<String>),
    Number(#[serde(rename = "number")] Option<Number>),
    Boolean(#[serde(rename = "boolean")] Option<bool>),
    Date(#[serde(rename = "date")] Option<DateTime<Utc>>),
}

/// Relation property value objects contain an array of page references within the relation property.
/// A page reference is an object with an id property,
/// with a string value (UUIDv4) corresponding to a page ID in another database.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RelationValue {
    id: PageId,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RollupValue {
    Number(#[serde(rename = "number")] Option<Number>),
    Date(#[serde(rename = "date")] Option<DateTime<Utc>>),
    // Todo: these property values don't have id properties...
    //       so this likely wont deserialize. would like to minimize duplicated code...
    Array(#[serde(rename = "array")] Vec<PropertyValue>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FileReference {
    name: String,
    url: String,
    mime_type: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PropertyValue {
    // https://developers.notion.com/reference/page#title-property-values
    Title {
        id: PropertyId,
        title: Vec<RichText>,
    },
    /// https://developers.notion.com/reference/page#rich-text-property-values
    #[serde(rename = "rich_text")]
    Text {
        id: PropertyId,
        rich_text: Vec<RichText>,
    },
    /// https://developers.notion.com/reference/page#number-property-values
    Number {
        id: PropertyId,
        number: Number,
    },
    /// https://developers.notion.com/reference/page#select-property-values
    Select {
        id: PropertyId,
        select: SelectedValue,
    },
    MultiSelect {
        id: PropertyId,
        multi_select: Vec<SelectedValue>,
    },
    Date {
        id: PropertyId,
        date: DateValue,
    },
    /// https://developers.notion.com/reference/page#formula-property-values
    Formula {
        id: PropertyId,
        formula: FormulaResultValue,
    },
    /// https://developers.notion.com/reference/page#relation-property-values
    Relation {
        id: PropertyId,
        relation: RelationValue,
    },
    Rollup {
        id: PropertyId,
        relation: Rollup,
    },
    People {
        id: PropertyId,
        people: Vec<User>,
    },
    Files {
        id: PropertyId,
        files: Vec<FileReference>,
    },
    Checkbox {
        id: PropertyId,
        checkbox: bool,
    },
    URL {
        id: PropertyId,
        url: String,
    },
    Email {
        id: PropertyId,
        email: String,
    },
    PhoneNumber {
        id: PropertyId,
        phone_number: String,
    },
    CreatedTime {
        id: PropertyId,
        created_time: DateTime<Utc>,
    },
    CreatedBy {
        id: PropertyId,
        created_by: User,
    },
    LastEditedTime {
        id: PropertyId,
        last_edited_time: DateTime<Utc>,
    },
    LastEditedBy {
        id: PropertyId,
        last_edited_by: User,
    },
}
