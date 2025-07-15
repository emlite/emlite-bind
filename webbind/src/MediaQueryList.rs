use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaQueryList {
    inner: EventTarget,
}
impl FromVal for MediaQueryList {
    fn from_val(v: &emlite::Val) -> Self {
        MediaQueryList {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaQueryList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaQueryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaQueryList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaQueryList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaQueryList> for emlite::Val {
    fn from(s: MediaQueryList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaQueryList);

impl MediaQueryList {
    pub fn media(&self) -> CSSOMString {
        self.inner.get("media").as_::<CSSOMString>()
    }
}
impl MediaQueryList {
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
impl MediaQueryList {
    pub fn add_listener(&self, callback: Function) -> Undefined {
        self.inner
            .call("addListener", &[callback.into()])
            .as_::<Undefined>()
    }
}
impl MediaQueryList {
    pub fn remove_listener(&self, callback: Function) -> Undefined {
        self.inner
            .call("removeListener", &[callback.into()])
            .as_::<Undefined>()
    }
}
impl MediaQueryList {
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: Any) {
        self.inner.set("onchange", value);
    }
}
