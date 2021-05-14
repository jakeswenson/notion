use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct PagingCursor(String);

#[derive(Serialize, Debug, Eq, PartialEq, Default)]
pub struct Paging {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<PagingCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,
}
