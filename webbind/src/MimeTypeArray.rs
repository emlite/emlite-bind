use super::*;

#[derive(Clone, Debug)]
pub struct MimeTypeArray {
    inner: emlite::Val,
}
impl FromVal for MimeTypeArray {
    fn from_val(v: &emlite::Val) -> Self {
        MimeTypeArray {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MimeTypeArray {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MimeTypeArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MimeTypeArray> for emlite::Val {
    fn from(s: MimeTypeArray) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MimeTypeArray {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl MimeTypeArray {
    pub fn item(&self, index: u32) -> MimeType {
        self.inner.call("item", &[index.into()]).as_::<MimeType>()
    }
}
impl MimeTypeArray {
    pub fn named_item(&self, name: jsbind::DOMString) -> MimeType {
        self.inner
            .call("namedItem", &[name.into()])
            .as_::<MimeType>()
    }
}
