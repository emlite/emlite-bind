use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for NotRestoredReasonDetails {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotRestoredReasonDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NotRestoredReasonDetails {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NotRestoredReasonDetails {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NotRestoredReasonDetails> for emlite::Val {
    fn from(s: NotRestoredReasonDetails) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NotRestoredReasonDetails);

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
