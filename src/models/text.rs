use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
struct Annotations {
    bold: Option<bool>,
    code: Option<bool>,
    color: Option<TextColor>,
    italic: Option<bool>,
    strikethrough: Option<bool>,
    underline: Option<bool>,
}

/// Properties common on all rich text objects
/// See https://developers.notion.com/reference/rich-text#all-rich-text
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RichTextCommon {
    plain_text: String,
    href: Option<String>,
    annotations: Option<Annotations>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Link {
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Text {
    content: String,
    link: Option<String>,
}

/// Rich text objects contain data for displaying formatted text, mentions, and equations.
/// A rich text object also contains annotations for style information.
/// Arrays of rich text objects are used within property objects and property
/// value objects to create what a user sees as a single text value in Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
