use crate::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UserCommon {
    pub id: UserId,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(alias = "type")]
    pub ty: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Owner {
    #[serde(alias = "type")]
    ty: Option<String>,
    workspace: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Bot {
    pub owner: Option<Owner>,
    pub workspace_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum User {
    Person {
        #[serde(flatten)]
        common: UserCommon,
        person: Person,
    },
    Bot {
        #[serde(flatten)]
        common: UserCommon,
        bot: Option<Bot>,
    },
}

impl User {
    pub fn id(&self) -> &UserId {
        match self {
            User::Person { common, person: _ } => &common.id,
            User::Bot { common, bot: _ } => &common.id,
        }
    }
}
