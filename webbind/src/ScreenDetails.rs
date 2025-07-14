use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScreenDetails {
    inner: EventTarget,
}
impl FromVal for ScreenDetails {
    fn from_val(v: &emlite::Val) -> Self {
        ScreenDetails {
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
impl core::ops::Deref for ScreenDetails {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScreenDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScreenDetails> for emlite::Val {
    fn from(s: ScreenDetails) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScreenDetails {
    pub fn screens(&self) -> jsbind::FrozenArray<ScreenDetailed> {
        self.inner
            .get("screens")
            .as_::<jsbind::FrozenArray<ScreenDetailed>>()
    }
}
impl ScreenDetails {
    pub fn current_screen(&self) -> ScreenDetailed {
        self.inner.get("currentScreen").as_::<ScreenDetailed>()
    }
}
impl ScreenDetails {
    pub fn onscreenschange(&self) -> jsbind::Any {
        self.inner.get("onscreenschange").as_::<jsbind::Any>()
    }

    pub fn set_onscreenschange(&mut self, value: jsbind::Any) {
        self.inner.set("onscreenschange", value);
    }
}
impl ScreenDetails {
    pub fn oncurrentscreenchange(&self) -> jsbind::Any {
        self.inner.get("oncurrentscreenchange").as_::<jsbind::Any>()
    }

    pub fn set_oncurrentscreenchange(&mut self, value: jsbind::Any) {
        self.inner.set("oncurrentscreenchange", value);
    }
}
