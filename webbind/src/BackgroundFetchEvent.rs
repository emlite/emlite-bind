use super::*;

/// The BackgroundFetchEvent class.
/// [`BackgroundFetchEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchEvent {
    inner: ExtendableEvent,
}
impl FromVal for BackgroundFetchEvent {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BackgroundFetchEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BackgroundFetchEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BackgroundFetchEvent> for Any {
    fn from(s: BackgroundFetchEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BackgroundFetchEvent> for Any {
    fn from(s: &BackgroundFetchEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BackgroundFetchEvent);

impl BackgroundFetchEvent {
    /// The `new BackgroundFetchEvent(..)` constructor, creating a new BackgroundFetchEvent instance
    pub fn new(type_: &JsString, init: &BackgroundFetchEventInit) -> BackgroundFetchEvent {
        Self {
            inner: Any::global("BackgroundFetchEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl BackgroundFetchEvent {
    /// Getter of the `registration` attribute.
    /// [`BackgroundFetchEvent.registration`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchEvent/registration)
    pub fn registration(&self) -> BackgroundFetchRegistration {
        self.inner
            .get("registration")
            .as_::<BackgroundFetchRegistration>()
    }
}
