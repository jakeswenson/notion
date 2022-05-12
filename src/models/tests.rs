use crate::models::text::{Annotations, Link, RichText, RichTextCommon, Text, TextColor};
use crate::models::{ListResponse, Object, Page};

#[test]
fn deserialize_page() {
    let _page: Page = serde_json::from_str(include_str!("tests/page.json")).unwrap();
}

#[test]
fn deserialize_query_result() {
    let _page: ListResponse<Page> =
        serde_json::from_str(include_str!("tests/query_result.json")).unwrap();
}

#[test]
fn deserialize_number_format() {
    let _search_results: ListResponse<Object> =
        serde_json::from_str(include_str!("tests/issue_15.json")).unwrap();
}

#[test]
fn rich_text() {
    let rich_text: RichText = serde_json::from_str(include_str!("tests/rich_text_text.json")).unwrap();
    assert_eq!(rich_text, RichText::Text {
        rich_text: RichTextCommon {
            plain_text: "Rich".to_string(),
            href: Some("https://github.com/jakeswenson/notion".to_string()),
            annotations: Some(Annotations {
                bold: Some(true),
                code: Some(true),
                color: Some(TextColor::Default),
                italic: Some(true),
                strikethrough: Some(true),
                underline: Some(true),
            }),
        },
        text: Text {
            content: "Rich".to_string(),
            link: Some(Link {
                url: "https://github.com/jakeswenson/notion".to_string()
            }),
        },
    })
}