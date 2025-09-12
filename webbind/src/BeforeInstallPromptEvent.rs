use super::*;

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
    pub fn new(type_: &JsString) -> BeforeInstallPromptEvent {
        Self {
            inner: Any::global("BeforeInstallPromptEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl BeforeInstallPromptEvent {
    /// The `new BeforeInstallPromptEvent(..)` constructor, creating a new BeforeInstallPromptEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &EventInit,
    ) -> BeforeInstallPromptEvent {
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
    pub fn prompt(&self) -> Promise<PromptResponseObject> {
        self.inner
            .call("prompt", &[])
            .as_::<Promise<PromptResponseObject>>()
    }
}
