pub mod paging;
pub mod properties;
pub mod search;
pub mod text;

use crate::models::properties::{PropertyConfiguration, PropertyValue};
use crate::models::text::RichText;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::paging::PagingCursor;
use crate::AsIdentifier;
pub use chrono::{DateTime, Utc};
pub use serde_json::value::Number;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
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

impl AsIdentifier<DatabaseId> for Database {
    fn id(&self) -> DatabaseId {
        self.id.clone()
    }
}

impl Database {
    pub fn title_plain_text(&self) -> String {
        self.title
            .iter()
            .flat_map(|rich_text| rich_text.plain_text().chars())
            .collect()
    }
}

/// https://developers.notion.com/reference/pagination#responses
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ListResponse<T> {
    pub results: Vec<T>,
    pub next_cursor: Option<PagingCursor>,
    pub has_more: bool,
}

impl<T> ListResponse<T> {
    pub fn results(&self) -> &[T] {
        &self.results
    }
}

impl ListResponse<Object> {
    pub fn only_databases(self) -> ListResponse<Database> {
        let databases = self
            .results
            .into_iter()
            .filter_map(|object| match object {
                Object::Database { database } => Some(database),
                _ => None,
            })
            .collect();

        ListResponse {
            results: databases,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        }
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
    Database {
        database_id: DatabaseId,
    },
    #[serde(rename = "page_id")]
    Page {
        page_id: PageId,
    },
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
pub struct BlockCommon {
    id: BlockId,
    created_time: DateTime<Utc>,
    last_edited_time: DateTime<Utc>,
    has_children: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextAndChildren {
    text: Vec<RichText>,
    children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Text {
    text: Vec<RichText>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToDoFields {
    text: Vec<RichText>,
    checked: bool,
    children: Option<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPageFields {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Block {
    Paragraph {
        #[serde(flatten)]
        common: BlockCommon,
        paragraph: TextAndChildren,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        #[serde(flatten)]
        common: BlockCommon,
        heading_1: Text,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        #[serde(flatten)]
        common: BlockCommon,
        heading_2: Text,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        #[serde(flatten)]
        common: BlockCommon,
        heading_3: Text,
    },
    BulletedListItem {
        #[serde(flatten)]
        common: BlockCommon,
        bulleted_list_item: TextAndChildren,
    },
    NumberedListItem {
        #[serde(flatten)]
        common: BlockCommon,
        numbered_list_item: TextAndChildren,
    },
    ToDo {
        #[serde(flatten)]
        common: BlockCommon,
        to_do: ToDoFields,
    },
    Toggle {
        #[serde(flatten)]
        common: BlockCommon,
        toggle: TextAndChildren,
    },
    ChildPage {
        #[serde(flatten)]
        common: BlockCommon,
        child_page: ChildPageFields,
    },
    #[serde(other)]
    Unsupported,
}

impl AsIdentifier<BlockId> for Block {
    fn id(&self) -> BlockId {
        use Block::*;
        match self {
            Paragraph { common, .. }
            | Heading1 { common, .. }
            | Heading2 { common, .. }
            | Heading3 { common, .. }
            | BulletedListItem { common, .. }
            | NumberedListItem { common, .. }
            | ToDo { common, .. }
            | Toggle { common, .. }
            | ChildPage { common, .. } => common.id.clone(),
            Unsupported {} => {
                panic!("Trying to reference identifier for unsupported block!")
            }
        }
    }
}

impl AsIdentifier<PageId> for Page {
    fn id(&self) -> PageId {
        self.id.clone()
    }
}

impl AsIdentifier<BlockId> for Page {
    fn id(&self) -> BlockId {
        self.id.clone().into()
    }
}

#[derive(Eq, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "object")]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Block {
        #[serde(flatten)]
        block: Block,
    },
    Database {
        #[serde(flatten)]
        database: Database,
    },
    Page {
        #[serde(flatten)]
        page: Page,
    },
    List {
        #[serde(flatten)]
        list: ListResponse<Object>,
    },
    User {
        #[serde(flatten)]
        user: User,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(transparent)]
pub struct BlockId(String);

impl BlockId {
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl From<PageId> for BlockId {
    fn from(page_id: PageId) -> Self {
        BlockId(page_id.0)
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Object {
    pub fn is_database(&self) -> bool {
        matches!(self, Object::Database { .. })
    }
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
    use crate::models::{ListResponse, Page};

    #[test]
    fn deserialize_page() {
        let _page: Page = serde_json::from_str(include_str!("models/tests/page.json")).unwrap();
    }

    #[test]
    fn deserialize_query_result() {
        let _page: ListResponse<Page> =
            serde_json::from_str(include_str!("models/tests/query_result.json")).unwrap();
    }
}
