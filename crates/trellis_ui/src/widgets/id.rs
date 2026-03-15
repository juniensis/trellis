use std::{
    borrow::Cow,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(Identifier);

impl Id {
    pub const fn new(id: &'static str) -> Self {
        Self(Identifier::Custom(Cow::Borrowed(id)))
    }
    pub fn unique() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Self(Identifier::Unique(id))
    }
}

impl From<&'static str> for Id {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(Identifier::Custom(Cow::Owned(value)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Identifier {
    Unique(usize),
    Custom(Cow<'static, str>),
}
