use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentTimelineOptions {
    inner: Any,
}
impl FromVal for DocumentTimelineOptions {
    fn from_val(v: &Any) -> Self {
        DocumentTimelineOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentTimelineOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentTimelineOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DocumentTimelineOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DocumentTimelineOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DocumentTimelineOptions> for Any {
    fn from(s: DocumentTimelineOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DocumentTimelineOptions> for Any {
    fn from(s: &DocumentTimelineOptions) -> Any {
        s.inner.clone()
    }
}

impl DocumentTimelineOptions {
    pub fn origin_time(&self) -> Any {
        self.inner.get("originTime").as_::<Any>()
    }

    pub fn set_origin_time(&mut self, value: &Any) {
        self.inner.set("originTime", value);
    }
}
