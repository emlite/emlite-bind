use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for BackgroundFetchUIOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BackgroundFetchUIOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
    pub fn icons(&self) -> Sequence<Any> {
        self.inner.get("icons").as_::<Sequence<Any>>()
    }

    pub fn set_icons(&mut self, value: Sequence<Any>) {
        self.inner.set("icons", value);
    }
}
impl BackgroundFetchUIOptions {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }

    pub fn set_title(&mut self, value: DOMString) {
        self.inner.set("title", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for BackgroundFetchUpdateUIEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BackgroundFetchUpdateUIEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(BackgroundFetchUpdateUIEvent);

impl BackgroundFetchUpdateUIEvent {
    pub fn new(type_: DOMString, init: Any) -> BackgroundFetchUpdateUIEvent {
        Self {
            inner: emlite::Val::global("BackgroundFetchUpdateUIEvent")
                .new(&[type_.into(), init.into()])
                .as_::<BackgroundFetchEvent>(),
        }
    }
}
impl BackgroundFetchUpdateUIEvent {
    pub fn update_ui0(&self) -> Promise {
        self.inner.call("updateUI", &[]).as_::<Promise>()
    }

    pub fn update_ui1(&self, options: BackgroundFetchUIOptions) -> Promise {
        self.inner
            .call("updateUI", &[options.into()])
            .as_::<Promise>()
    }
}
