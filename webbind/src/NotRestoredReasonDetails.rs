use super::*;

#[derive(Clone, Debug)]
pub struct NotRestoredReasonDetails {
    inner: emlite::Val,
}
impl FromVal for NotRestoredReasonDetails {
    fn from_val(v: &emlite::Val) -> Self {
        NotRestoredReasonDetails {
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
impl std::ops::Deref for NotRestoredReasonDetails {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NotRestoredReasonDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NotRestoredReasonDetails> for emlite::Val {
    fn from(s: NotRestoredReasonDetails) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NotRestoredReasonDetails {
    pub fn reason(&self) -> jsbind::DOMString {
        self.inner.get("reason").as_::<jsbind::DOMString>()
    }
}
impl NotRestoredReasonDetails {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
