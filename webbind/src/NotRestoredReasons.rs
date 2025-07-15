use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotRestoredReasons {
    inner: emlite::Val,
}
impl FromVal for NotRestoredReasons {
    fn from_val(v: &emlite::Val) -> Self {
        NotRestoredReasons {
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
impl core::ops::Deref for NotRestoredReasons {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotRestoredReasons {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NotRestoredReasons {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NotRestoredReasons {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NotRestoredReasons> for emlite::Val {
    fn from(s: NotRestoredReasons) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NotRestoredReasons);

impl NotRestoredReasons {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }
}
impl NotRestoredReasons {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
impl NotRestoredReasons {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl NotRestoredReasons {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }
}
impl NotRestoredReasons {
    pub fn reasons(&self) -> FrozenArray<NotRestoredReasonDetails> {
        self.inner
            .get("reasons")
            .as_::<FrozenArray<NotRestoredReasonDetails>>()
    }
}
impl NotRestoredReasons {
    pub fn children(&self) -> FrozenArray<NotRestoredReasons> {
        self.inner
            .get("children")
            .as_::<FrozenArray<NotRestoredReasons>>()
    }
}
impl NotRestoredReasons {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
