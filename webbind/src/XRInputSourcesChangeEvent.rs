use super::*;

/// The XRInputSourcesChangeEvent class.
/// [`XRInputSourcesChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourcesChangeEvent {
    inner: Event,
}

impl FromVal for XRInputSourcesChangeEvent {
    fn from_val(v: &Any) -> Self {
        XRInputSourcesChangeEvent {
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

impl core::ops::Deref for XRInputSourcesChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRInputSourcesChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRInputSourcesChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRInputSourcesChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRInputSourcesChangeEvent> for Any {
    fn from(s: XRInputSourcesChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRInputSourcesChangeEvent> for Any {
    fn from(s: &XRInputSourcesChangeEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRInputSourcesChangeEvent);

impl XRInputSourcesChangeEvent {
    /// The `new XRInputSourcesChangeEvent(..)` constructor, creating a new XRInputSourcesChangeEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &XRInputSourcesChangeEventInit,
    ) -> XRInputSourcesChangeEvent {
        Self {
            inner: Any::global("XRInputSourcesChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRInputSourcesChangeEvent {
    /// Getter of the `session` attribute.
    /// [`XRInputSourcesChangeEvent.session`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/session)
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }
}
impl XRInputSourcesChangeEvent {
    /// Getter of the `added` attribute.
    /// [`XRInputSourcesChangeEvent.added`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/added)
    pub fn added(&self) -> TypedArray<XRInputSource> {
        self.inner.get("added").as_::<TypedArray<XRInputSource>>()
    }
}
impl XRInputSourcesChangeEvent {
    /// Getter of the `removed` attribute.
    /// [`XRInputSourcesChangeEvent.removed`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourcesChangeEvent/removed)
    pub fn removed(&self) -> TypedArray<XRInputSource> {
        self.inner.get("removed").as_::<TypedArray<XRInputSource>>()
    }
}
