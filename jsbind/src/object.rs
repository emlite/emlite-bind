use crate::utils::bind;
use emlite::FromVal;

pub struct Object {
    inner: emlite::Val,
}

impl Object {
    pub fn new() -> Object {
        Self {
            inner: emlite::Val::object(),
        }
    }
    pub fn is_null(&self) -> bool {
        self.inner.as_handle() == emlite::EmlitePredefHandles::Null as u32
    }
    pub fn is_undefined(&self) -> bool {
        self.inner.as_handle() == emlite::EmlitePredefHandles::Undefined as u32
    }
    pub fn has_own_property(&self, prop: &str) -> bool {
        self.inner.has_own_property(prop)
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

bind!(Object);
