use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BackgroundFetchUIOptions {
    inner: emlite::Val,
}
impl FromVal for BackgroundFetchUIOptions {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchUIOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchUIOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchUIOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchUIOptions> for emlite::Val {
    fn from(s: BackgroundFetchUIOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchUIOptions {
    pub fn icons(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("icons")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_icons(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("icons", value);
    }
}
impl BackgroundFetchUIOptions {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }

    pub fn set_title(&mut self, value: jsbind::DOMString) {
        self.inner.set("title", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BackgroundFetchUpdateUIEvent {
    inner: BackgroundFetchEvent,
}
impl FromVal for BackgroundFetchUpdateUIEvent {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchUpdateUIEvent {
            inner: BackgroundFetchEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchUpdateUIEvent {
    type Target = BackgroundFetchEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchUpdateUIEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchUpdateUIEvent> for emlite::Val {
    fn from(s: BackgroundFetchUpdateUIEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchUpdateUIEvent {
    pub fn new(type_: jsbind::DOMString, init: jsbind::Any) -> BackgroundFetchUpdateUIEvent {
        Self {
            inner: emlite::Val::global("BackgroundFetchUpdateUIEvent")
                .new(&[type_.into(), init.into()])
                .as_::<BackgroundFetchEvent>(),
        }
    }
}
impl BackgroundFetchUpdateUIEvent {
    pub fn update_ui0(&self) -> jsbind::Promise {
        self.inner.call("updateUI", &[]).as_::<jsbind::Promise>()
    }

    pub fn update_ui1(&self, options: BackgroundFetchUIOptions) -> jsbind::Promise {
        self.inner
            .call("updateUI", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
