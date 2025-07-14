use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XMLDocument {
    inner: Document,
}
impl FromVal for XMLDocument {
    fn from_val(v: &emlite::Val) -> Self {
        XMLDocument {
            inner: Document::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XMLDocument {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XMLDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XMLDocument> for emlite::Val {
    fn from(s: XMLDocument) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
