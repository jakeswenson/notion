use crate::ids::UserId;
use crate::models::properties::{DateOrDateTime, DateValue};
use crate::models::text::{
    Annotations, Link, MentionObject, RichText, RichTextCommon, Text, TextColor,
};
use crate::models::users::{Person, User, UserCommon};
use crate::models::{
    BlockCommon, Callout, ExternalFileObject, FileOrEmojiObject, InternalFileObject, ListResponse,
    Object, Page,
};
use crate::{models, Block, BlockId};
use chrono::{DateTime, NaiveDate};
use std::str::FromStr;

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
    let rich_text_text: RichText =
        serde_json::from_str(include_str!("tests/rich_text_text.json")).unwrap();
    assert_eq!(
        rich_text_text,
        RichText::Text {
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
        }
    )
}

#[test]
fn rich_text_mention_user_person() {
    let rich_text_mention_user_person: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_user_person.json")).unwrap();
    assert_eq!(rich_text_mention_user_person, RichText::Mention {
        rich_text: RichTextCommon {
            plain_text: "@John Doe".to_string(),
            href: None,
            annotations: Some(Annotations {
                bold: Some(false),
                code: Some(false),
                color: Some(TextColor::Default),
                italic: Some(false),
                strikethrough: Some(false),
                underline: Some(false),
            }),
        },
        mention: MentionObject::User {
            user: User::Person {
                common: UserCommon {
                    id: UserId::from_str("1118608e-35e8-4fa3-aef7-a4ced85ce8e0").unwrap(),
                    name: Some("John Doe".to_string()),
                    avatar_url: Some("https://secure.notion-static.com/e6a352a8-8381-44d0-a1dc-9ed80e62b53d.jpg".to_string()),
                },
                person: Person { email: "john.doe@gmail.com".to_string() },
            }
        },
    })
}

#[test]
fn rich_text_mention_date() {
    let rich_text_mention_date: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_date.json")).unwrap();
    assert_eq!(
        rich_text_mention_date,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-04-16 → ".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::Date(NaiveDate::from_str("2022-04-16").unwrap()),
                    end: None,
                    time_zone: None,
                }
            },
        }
    )
}

#[test]
fn rich_text_mention_date_with_time() {
    let rich_text_mention_date_with_time: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_date_with_time.json")).unwrap();
    assert_eq!(
        rich_text_mention_date_with_time,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-05-14T09:00:00.000-04:00 → ".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::DateTime(
                        DateTime::from_str("2022-05-14T09:00:00.000-04:00").unwrap()
                    ),
                    end: None,
                    time_zone: None,
                }
            },
        }
    )
}

#[test]
fn rich_text_mention_date_with_end() {
    let rich_text_mention_date_with_end: RichText =
        serde_json::from_str(include_str!("tests/rich_text_mention_date_with_end.json")).unwrap();
    assert_eq!(
        rich_text_mention_date_with_end,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-05-12 → 2022-05-13".to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::Date(NaiveDate::from_str("2022-05-12").unwrap()),
                    end: Some(DateOrDateTime::Date(
                        NaiveDate::from_str("2022-05-13").unwrap()
                    )),
                    time_zone: None,
                }
            },
        }
    )
}

#[test]
fn rich_text_mention_date_with_end_and_time() {
    let rich_text_mention_date_with_end_and_time: RichText = serde_json::from_str(include_str!(
        "tests/rich_text_mention_date_with_end_and_time.json"
    ))
    .unwrap();
    assert_eq!(
        rich_text_mention_date_with_end_and_time,
        RichText::Mention {
            rich_text: RichTextCommon {
                plain_text: "2022-04-16T12:00:00.000-04:00 → 2022-04-16T12:00:00.000-04:00"
                    .to_string(),
                href: None,
                annotations: Some(Annotations {
                    bold: Some(false),
                    code: Some(false),
                    color: Some(TextColor::Default),
                    italic: Some(false),
                    strikethrough: Some(false),
                    underline: Some(false),
                }),
            },
            mention: MentionObject::Date {
                date: DateValue {
                    start: DateOrDateTime::DateTime(
                        DateTime::from_str("2022-04-16T12:00:00.000-04:00").unwrap()
                    ),
                    end: Some(DateOrDateTime::DateTime(
                        DateTime::from_str("2022-04-16T12:00:00.000-04:00").unwrap()
                    )),
                    time_zone: None,
                }
            },
        }
    )
}

#[test]
fn heading_1() {
    let heading_1: Block = serde_json::from_str(include_str!("tests/heading_1.json")).unwrap();
    assert_eq!(
        heading_1,
        Block::Heading1 {
            common: BlockCommon {
                id: BlockId::from_str("9e891834-6a03-475c-a2b8-421e17f0f3aa").unwrap(),
                created_time: DateTime::from_str("2022-05-12T21:15:00.000Z").unwrap(),
                last_edited_time: DateTime::from_str("2022-05-12T22:10:00.000Z").unwrap(),
                has_children: false,
                created_by: UserCommon {
                    id: UserId::from_str("6419f912-5293-4ea8-b2c8-9c3ce44f90e3").unwrap(),
                    name: None,
                    avatar_url: None,
                },
                last_edited_by: UserCommon {
                    id: UserId::from_str("6419f912-5293-4ea8-b2c8-9c3ce44f90e3").unwrap(),
                    name: None,
                    avatar_url: None,
                },
            },
            heading_1: models::Text {
                rich_text: vec![
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "This".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(true),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: "This".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: " ".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: " ".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "is".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(true),
                            }),
                        },
                        text: Text {
                            content: "is".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: " ".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: " ".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "a".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(true),
                                strikethrough: Some(false),
                                underline: Some(true),
                            }),
                        },
                        text: Text {
                            content: "a".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: " ".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: " ".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "Heading".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(true),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: "Heading".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: " ".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: " ".to_string(),
                            link: None,
                        },
                    },
                    RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "1".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(true),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: "1".to_string(),
                            link: None,
                        },
                    },
                ]
            },
        }
    )
}

#[test]
fn emoji_object() {
    let emoji_object: FileOrEmojiObject =
        serde_json::from_str(include_str!("tests/emoji_object.json")).unwrap();
    assert_eq!(
        emoji_object,
        FileOrEmojiObject::Emoji {
            emoji: "💡".to_string()
        }
    )
}

#[test]
fn file_object() {
    let file_object: FileOrEmojiObject =
        serde_json::from_str(include_str!("tests/file_object.json")).unwrap();
    assert_eq!(file_object, FileOrEmojiObject::File {
        file: InternalFileObject {
            url: "https://s3.us-west-2.amazonaws.com/secure.notion-static.com/2703e742-ace5-428c-a74d-1c587ceddc32/DiRT_Rally.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Content-Sha256=UNSIGNED-PAYLOAD&X-Amz-Credential=AKIAT73L2G45EIPT3X45%2F20220513%2Fus-west-2%2Fs3%2Faws4_request&X-Amz-Date=20220513T201035Z&X-Amz-Expires=3600&X-Amz-Signature=714b49bde0b499fb8f3aae1a88a8cbd374f2b09c1d128e91cac49e85ce0e00fb&X-Amz-SignedHeaders=host&x-id=GetObject".to_string(),
            expiry_time: DateTime::from_str("2022-05-13T21:10:35.817Z").unwrap(),
        }
    })
}

#[test]
fn external_file_object() {
    let external_file_object: FileOrEmojiObject =
        serde_json::from_str(include_str!("tests/external_file_object.json")).unwrap();
    assert_eq!(
        external_file_object,
        FileOrEmojiObject::External {
            external: ExternalFileObject {
                url: "https://nerdist.com/wp-content/uploads/2020/07/maxresdefault.jpg".to_string(),
            }
        }
    )
}

#[test]
fn callout() {
    let callout: Object = serde_json::from_str(include_str!("tests/callout.json")).unwrap();
    assert_eq!(
        callout,
        Object::Block {
            block: Block::Callout {
                common: BlockCommon {
                    id: BlockId::from_str("00e8829a-a7b8-4075-884a-8f53be145d2f").unwrap(),
                    created_time: DateTime::from_str("2022-05-13T20:08:00.000Z").unwrap(),
                    last_edited_time: DateTime::from_str("2022-05-13T20:08:00.000Z").unwrap(),
                    has_children: true,
                    created_by: UserCommon {
                        id: UserId::from_str("e2507360-468c-4e0f-a928-7bbcbbb45353").unwrap(),
                        name: None,
                        avatar_url: None,
                    },
                    last_edited_by: UserCommon {
                        id: UserId::from_str("e2507360-468c-4e0f-a928-7bbcbbb45353").unwrap(),
                        name: None,
                        avatar_url: None,
                    },
                },
                callout: Callout {
                    rich_text: vec![RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "Test callout".to_string(),
                            href: None,
                            annotations: Some(Annotations {
                                bold: Some(false),
                                code: Some(false),
                                color: Some(TextColor::Default),
                                italic: Some(false),
                                strikethrough: Some(false),
                                underline: Some(false),
                            }),
                        },
                        text: Text {
                            content: "Test callout".to_string(),
                            link: None
                        },
                    }],
                    icon: FileOrEmojiObject::Emoji {
                        emoji: "💡".to_string()
                    },
                    color: TextColor::Green,
                },
            }
        }
    )
}
