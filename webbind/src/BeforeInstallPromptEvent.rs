use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PromptResponseObject {
    inner: Any,
}
impl FromVal for PromptResponseObject {
    fn from_val(v: &Any) -> Self {
        PromptResponseObject { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PromptResponseObject {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PromptResponseObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PromptResponseObject {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PromptResponseObject {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PromptResponseObject> for Any {
    fn from(s: PromptResponseObject) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PromptResponseObject> for Any {
    fn from(s: &PromptResponseObject) -> Any {
        s.inner.clone()
    }
}

impl PromptResponseObject {
    pub fn user_choice(&self) -> AppBannerPromptOutcome {
        self.inner.get("userChoice").as_::<AppBannerPromptOutcome>()
    }

    pub fn set_user_choice(&mut self, value: &AppBannerPromptOutcome) {
        self.inner.set("userChoice", value);
    }
}
/// The BeforeInstallPromptEvent class.
/// [`BeforeInstallPromptEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BeforeInstallPromptEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BeforeInstallPromptEvent {
    inner: Event,
}
impl FromVal for BeforeInstallPromptEvent {
    fn from_val(v: &Any) -> Self {
        BeforeInstallPromptEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BeforeInstallPromptEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BeforeInstallPromptEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BeforeInstallPromptEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BeforeInstallPromptEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BeforeInstallPromptEvent> for Any {
    fn from(s: BeforeInstallPromptEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BeforeInstallPromptEvent> for Any {
    fn from(s: &BeforeInstallPromptEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BeforeInstallPromptEvent);

impl BeforeInstallPromptEvent {
    /// The `new BeforeInstallPromptEvent(..)` constructor, creating a new BeforeInstallPromptEvent instance
    pub fn new0(type_: &str) -> BeforeInstallPromptEvent {
        Self {
            inner: Any::global("BeforeInstallPromptEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new BeforeInstallPromptEvent(..)` constructor, creating a new BeforeInstallPromptEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> BeforeInstallPromptEvent {
        Self {
            inner: Any::global("BeforeInstallPromptEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl BeforeInstallPromptEvent {
    /// The prompt method.
    /// [`BeforeInstallPromptEvent.prompt`](https://developer.mozilla.org/en-US/docs/Web/API/BeforeInstallPromptEvent/prompt)
    pub fn prompt(&self) -> Promise {
        self.inner.call("prompt", &[]).as_::<Promise>()
    }
}
