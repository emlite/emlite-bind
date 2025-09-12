use super::*;

/// The BackgroundFetchUpdateUIEvent class.
/// [`BackgroundFetchUpdateUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchUpdateUIEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchUpdateUIEvent {
    inner: BackgroundFetchEvent,
}

impl FromVal for BackgroundFetchUpdateUIEvent {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchUpdateUIEvent {
            inner: BackgroundFetchEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for BackgroundFetchUpdateUIEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BackgroundFetchUpdateUIEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BackgroundFetchUpdateUIEvent> for Any {
    fn from(s: BackgroundFetchUpdateUIEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BackgroundFetchUpdateUIEvent> for Any {
    fn from(s: &BackgroundFetchUpdateUIEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BackgroundFetchUpdateUIEvent);

impl BackgroundFetchUpdateUIEvent {
    /// The `new BackgroundFetchUpdateUIEvent(..)` constructor, creating a new BackgroundFetchUpdateUIEvent instance
    pub fn new(type_: &JsString, init: &BackgroundFetchEventInit) -> BackgroundFetchUpdateUIEvent {
        Self {
            inner: Any::global("BackgroundFetchUpdateUIEvent")
                .new(&[type_.into(), init.into()])
                .as_::<BackgroundFetchEvent>(),
        }
    }
}

impl BackgroundFetchUpdateUIEvent {
    /// The updateUI method.
    /// [`BackgroundFetchUpdateUIEvent.updateUI`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchUpdateUIEvent/updateUI)
    pub fn update_ui(&self) -> Promise<Undefined> {
        self.inner.call("updateUI", &[]).as_::<Promise<Undefined>>()
    }
}
impl BackgroundFetchUpdateUIEvent {
    /// The updateUI method.
    /// [`BackgroundFetchUpdateUIEvent.updateUI`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchUpdateUIEvent/updateUI)
    pub fn update_ui_with_options(&self, options: &BackgroundFetchUIOptions) -> Promise<Undefined> {
        self.inner
            .call("updateUI", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
