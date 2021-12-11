use std::fmt::Display;

pub trait Identifier: Display {
    fn value(&self) -> &str;
}

/// Meant to be a helpful trait allowing anything that can be
/// identified by the type specified in `ById`.
pub trait AsIdentifier<ById: Identifier> {
    fn as_id(&self) -> &ById;
}

impl<T> AsIdentifier<T> for T
where
    T: Identifier,
{
    fn as_id(&self) -> &T {
        &self
    }
}

impl<T> AsIdentifier<T> for &T
where
    T: Identifier,
{
    fn as_id(&self) -> &T {
        self
    }
}

macro_rules! identifier {
    ($name:ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: String) -> Self {
                $name(value)
            }
        }

        impl Identifier for $name {
            fn value(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

identifier!(DatabaseId);
identifier!(PageId);
identifier!(BlockId);
identifier!(UserId);
identifier!(PropertyId);

impl From<PageId> for BlockId {
    fn from(page_id: PageId) -> Self {
        BlockId(page_id.0)
    }
}
