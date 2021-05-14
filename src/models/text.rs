use serde::{Deserialize, Serialize};

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
pub struct RichTextCommon {
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
