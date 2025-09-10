use super::*;

/// The XRInputSourceEvent class.
/// [`XRInputSourceEvent`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourceEvent {
    inner: Event,
}

impl FromVal for XRInputSourceEvent {
    fn from_val(v: &Any) -> Self {
        XRInputSourceEvent {
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

impl core::ops::Deref for XRInputSourceEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRInputSourceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRInputSourceEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRInputSourceEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRInputSourceEvent> for Any {
    fn from(s: XRInputSourceEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRInputSourceEvent> for Any {
    fn from(s: &XRInputSourceEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRInputSourceEvent);

impl XRInputSourceEvent {
    /// The `new XRInputSourceEvent(..)` constructor, creating a new XRInputSourceEvent instance
    pub fn new(type_: &JsString, event_init_dict: &XRInputSourceEventInit) -> XRInputSourceEvent {
        Self {
            inner: Any::global("XRInputSourceEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRInputSourceEvent {
    /// Getter of the `frame` attribute.
    /// [`XRInputSourceEvent.frame`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/frame)
    pub fn frame(&self) -> XRFrame {
        self.inner.get("frame").as_::<XRFrame>()
    }
}
impl XRInputSourceEvent {
    /// Getter of the `inputSource` attribute.
    /// [`XRInputSourceEvent.inputSource`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSourceEvent/inputSource)
    pub fn input_source(&self) -> XRInputSource {
        self.inner.get("inputSource").as_::<XRInputSource>()
    }
}
