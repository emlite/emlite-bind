use super::*;

/// The PromiseRejectionEvent class.
/// [`PromiseRejectionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PromiseRejectionEvent {
    inner: Event,
}
impl FromVal for PromiseRejectionEvent {
    fn from_val(v: &Any) -> Self {
        PromiseRejectionEvent {
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
impl core::ops::Deref for PromiseRejectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PromiseRejectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PromiseRejectionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PromiseRejectionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PromiseRejectionEvent> for Any {
    fn from(s: PromiseRejectionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PromiseRejectionEvent> for Any {
    fn from(s: &PromiseRejectionEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PromiseRejectionEvent);

impl PromiseRejectionEvent {
    /// The `new PromiseRejectionEvent(..)` constructor, creating a new PromiseRejectionEvent instance
    pub fn new(type_: &JsString, event_init_dict: &Any) -> PromiseRejectionEvent {
        Self {
            inner: Any::global("PromiseRejectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PromiseRejectionEvent {
    /// Getter of the `promise` attribute.
    /// [`PromiseRejectionEvent.promise`](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/promise)
    pub fn promise(&self) -> Object {
        self.inner.get("promise").as_::<Object>()
    }
}
impl PromiseRejectionEvent {
    /// Getter of the `reason` attribute.
    /// [`PromiseRejectionEvent.reason`](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/reason)
    pub fn reason(&self) -> Any {
        self.inner.get("reason").as_::<Any>()
    }
}
