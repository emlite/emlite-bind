use super::*;

#[derive(Clone, Debug)]
pub struct PromptResponseObject {
    inner: emlite::Val,
}
impl FromVal for PromptResponseObject {
    fn from_val(v: &emlite::Val) -> Self {
        PromptResponseObject { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PromptResponseObject {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PromptResponseObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PromptResponseObject> for emlite::Val {
    fn from(s: PromptResponseObject) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PromptResponseObject {
    pub fn user_choice(&self) -> AppBannerPromptOutcome {
        self.inner.get("userChoice").as_::<AppBannerPromptOutcome>()
    }

    pub fn set_user_choice(&mut self, value: AppBannerPromptOutcome) {
        self.inner.set("userChoice", value);
    }
}
#[derive(Clone, Debug)]
pub struct BeforeInstallPromptEvent {
    inner: Event,
}
impl FromVal for BeforeInstallPromptEvent {
    fn from_val(v: &emlite::Val) -> Self {
        BeforeInstallPromptEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for BeforeInstallPromptEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BeforeInstallPromptEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BeforeInstallPromptEvent> for emlite::Val {
    fn from(s: BeforeInstallPromptEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BeforeInstallPromptEvent {
    pub fn new0(type_: jsbind::DOMString) -> BeforeInstallPromptEvent {
        Self {
            inner: emlite::Val::global("BeforeInstallPromptEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> BeforeInstallPromptEvent {
        Self {
            inner: emlite::Val::global("BeforeInstallPromptEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl BeforeInstallPromptEvent {
    pub fn prompt(&self) -> jsbind::Promise {
        self.inner.call("prompt", &[]).as_::<jsbind::Promise>()
    }
}
