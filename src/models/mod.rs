pub mod error;
pub mod paging;
pub mod properties;
pub mod search;
#[cfg(test)]
mod tests;
pub mod text;
pub mod users;

use crate::models::properties::{PropertyConfiguration, PropertyValue};
use crate::models::text::{RichText, TextColor};
use crate::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ids::{AsIdentifier, BlockId, DatabaseId, PageId};
use crate::models::error::ErrorResponse;
use crate::models::paging::PagingCursor;
use crate::models::users::{User, UserCommon};
pub use chrono::{DateTime, Utc};
pub use serde_json::value::Number;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum ObjectType {
    Database,
    List,
}

/// Represents a Notion Database
/// See <https://developers.notion.com/reference/database>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Database {
    /// Unique identifier for the database.
    pub id: DatabaseId,
    /// Date and time when this database was created.
    pub created_time: DateTime<Utc>,
    /// Date and time when this database was updated.
    pub last_edited_time: DateTime<Utc>,
    /// Name of the database as it appears in Notion.
    pub title: Vec<RichText>,
    /// Schema of properties for the database as they appear in Notion.
    //
    // key string
    // The name of the property as it appears in Notion.
    //
    // value object
    // A Property object.
    pub properties: HashMap<String, PropertyConfiguration>,
}

impl AsIdentifier<DatabaseId> for Database {
    fn as_id(&self) -> &DatabaseId {
        &self.id
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

/// <https://developers.notion.com/reference/pagination#responses>
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

    pub(crate) fn expect_databases(self) -> Result<ListResponse<Database>, crate::Error> {
        let databases: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Database { database } => Ok(database),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: databases?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
    }

    pub(crate) fn expect_pages(self) -> Result<ListResponse<Page>, crate::Error> {
        let items: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Page { page } => Ok(page),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: items?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
    }

    pub(crate) fn expect_blocks(self) -> Result<ListResponse<Block>, crate::Error> {
        let items: Result<Vec<_>, _> = self
            .results
            .into_iter()
            .map(|object| match object {
                Object::Block { block } => Ok(block),
                response => Err(Error::UnexpectedResponse { response }),
            })
            .collect();

        Ok(ListResponse {
            results: items?,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
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
    pub properties: HashMap<String, PropertyValue>,
}

impl Properties {
    pub fn title(&self) -> Option<String> {
        self.properties.values().find_map(|p| match p {
            PropertyValue::Title { title, .. } => {
                Some(title.into_iter().map(|t| t.plain_text()).collect())
            }
            _ => None,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub id: PageId,
    /// Date and time when this page was created.
    pub created_time: DateTime<Utc>,
    /// Date and time when this page was updated.
    pub last_edited_time: DateTime<Utc>,
    /// The archived status of the page.
    pub archived: bool,
    pub properties: Properties,
    pub parent: Parent,
}

impl Page {
    pub fn title(&self) -> Option<String> {
        self.properties.title()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockCommon {
    pub id: BlockId,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub has_children: bool,
    pub created_by: UserCommon,
    pub last_edited_by: UserCommon,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextAndChildren {
    pub rich_text: Vec<RichText>,
    pub children: Option<Vec<Block>>,
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Text {
    pub rich_text: Vec<RichText>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct InternalFileObject {
    url: String,
    expiry_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ExternalFileObject {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FileOrEmojiObject {
    Emoji { emoji: String },
    File { file: InternalFileObject },
    External { external: ExternalFileObject },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FileObject {
    File { file: InternalFileObject },
    External { external: ExternalFileObject },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Callout {
    pub rich_text: Vec<RichText>,
    pub icon: FileOrEmojiObject,
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToDoFields {
    pub rich_text: Vec<RichText>,
    pub checked: bool,
    pub children: Option<Vec<Block>>,
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPageFields {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabaseFields {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EmbedFields {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BookmarkFields {
    pub url: String,
    pub caption: Vec<RichText>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CodeLanguage {
    Abap,
    Arduino,
    Bash,
    Basic,
    C,
    Clojure,
    Coffeescript,
    #[serde(rename = "c++")]
    CPlusPlus,
    #[serde(rename = "c#")]
    CSharp,
    Css,
    Dart,
    Diff,
    Docker,
    Elixir,
    Elm,
    Erlang,
    Flow,
    Fortran,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    Glsl,
    Go,
    Graphql,
    Groovy,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    Livescript,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    Ocaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    PlainText,
    Powershell,
    Prolog,
    Protobuf,
    Python,
    R,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Sql,
    Swift,
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    Webassembly,
    Xml,
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaCAndCPlusPlusAndCSharp,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CodeFields {
    pub rich_text: Vec<RichText>,
    pub caption: Vec<RichText>,
    pub language: CodeLanguage,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Equation {
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableOfContents {
    pub color: TextColor,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnListFields {
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnFields {
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewFields {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TemplateFields {
    pub rich_text: Vec<RichText>,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum LinkToPageFields {
    PageId { page_id: PageId },
    DatabaseId { database_id: DatabaseId },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedFromObject {
    pub block_id: BlockId,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedBlockFields {
    pub synced_from: Option<SyncedFromObject>,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableFields {
    pub table_width: u64,
    pub has_column_header: bool,
    pub has_row_header: bool,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableRowFields {
    pub cells: Vec<RichText>,
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
    Callout {
        #[serde(flatten)]
        common: BlockCommon,
        callout: Callout,
    },
    Quote {
        #[serde(flatten)]
        common: BlockCommon,
        quote: TextAndChildren,
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
    Code {
        #[serde(flatten)]
        common: BlockCommon,
        code: CodeFields,
    },
    ChildPage {
        #[serde(flatten)]
        common: BlockCommon,
        child_page: ChildPageFields,
    },
    ChildDatabase {
        #[serde(flatten)]
        common: BlockCommon,
        child_page: ChildDatabaseFields,
    },
    Embed {
        #[serde(flatten)]
        common: BlockCommon,
        embed: EmbedFields,
    },
    Image {
        #[serde(flatten)]
        common: BlockCommon,
        image: FileObject,
    },
    Video {
        #[serde(flatten)]
        common: BlockCommon,
        video: FileObject,
    },
    File {
        #[serde(flatten)]
        common: BlockCommon,
        file: FileObject,
        caption: Text,
    },
    Pdf {
        #[serde(flatten)]
        common: BlockCommon,
        pdf: FileObject,
    },
    Bookmark {
        #[serde(flatten)]
        common: BlockCommon,
        bookmark: BookmarkFields,
    },
    Equation {
        #[serde(flatten)]
        common: BlockCommon,
        equation: Equation,
    },
    Divider {
        #[serde(flatten)]
        common: BlockCommon,
    },
    TableOfContents {
        #[serde(flatten)]
        common: BlockCommon,
        table_of_contents: TableOfContents,
    },
    Breadcrumb {
        #[serde(flatten)]
        common: BlockCommon,
    },
    ColumnList {
        #[serde(flatten)]
        common: BlockCommon,
        column_list: ColumnListFields,
    },
    Column {
        #[serde(flatten)]
        common: BlockCommon,
        column: ColumnFields,
    },
    LinkPreview {
        #[serde(flatten)]
        common: BlockCommon,
        link_preview: LinkPreviewFields,
    },
    Template {
        #[serde(flatten)]
        common: BlockCommon,
        template: TemplateFields,
    },
    LinkToPage {
        #[serde(flatten)]
        common: BlockCommon,
        link_to_page: LinkToPageFields,
    },
    Table {
        #[serde(flatten)]
        common: BlockCommon,
        table: TableFields,
    },
    SyncedBlock {
        #[serde(flatten)]
        common: BlockCommon,
        synced_block: SyncedBlockFields,
    },
    TableRow {
        #[serde(flatten)]
        common: BlockCommon,
        table_row: TableRowFields,
    },
    Unsupported {
        #[serde(flatten)]
        common: BlockCommon,
    },
    #[serde(other)]
    Unknown,
}

impl AsIdentifier<BlockId> for Block {
    fn as_id(&self) -> &BlockId {
        use Block::*;
        match self {
            Paragraph { common, .. }
            | Heading1 { common, .. }
            | Heading2 { common, .. }
            | Heading3 { common, .. }
            | Callout { common, .. }
            | Quote { common, .. }
            | BulletedListItem { common, .. }
            | NumberedListItem { common, .. }
            | ToDo { common, .. }
            | Toggle { common, .. }
            | Code { common, .. }
            | ChildPage { common, .. }
            | ChildDatabase { common, .. }
            | Embed { common, .. }
            | Image { common, .. }
            | Video { common, .. }
            | File { common, .. }
            | Pdf { common, .. }
            | Bookmark { common, .. }
            | Equation { common, .. }
            | Divider { common, .. }
            | TableOfContents { common, .. }
            | Breadcrumb { common, .. }
            | ColumnList { common, .. }
            | Column { common, .. }
            | LinkPreview { common, .. }
            | Template { common, .. }
            | LinkToPage { common, .. }
            | SyncedBlock { common, .. }
            | Table { common, .. }
            | TableRow { common, .. }
            | Unsupported { common, .. } => &common.id,
            Unknown => {
                panic!("Trying to reference identifier for unknown block!")
            }
        }
    }
}

impl AsIdentifier<PageId> for Page {
    fn as_id(&self) -> &PageId {
        &self.id
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
    Error {
        #[serde(flatten)]
        error: ErrorResponse,
    },
}

impl Object {
    pub fn is_database(&self) -> bool {
        matches!(self, Object::Database { .. })
    }
}
