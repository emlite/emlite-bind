use super::*;

/// The XRLayerEvent class.
/// [`XRLayerEvent`](https://developer.mozilla.org/en-US/docs/Web/API/XRLayerEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLayerEvent {
    inner: Event,
}
impl FromVal for XRLayerEvent {
    fn from_val(v: &Any) -> Self {
        XRLayerEvent {
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
impl core::ops::Deref for XRLayerEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLayerEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRLayerEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRLayerEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRLayerEvent> for Any {
    fn from(s: XRLayerEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRLayerEvent> for Any {
    fn from(s: &XRLayerEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRLayerEvent);

impl XRLayerEvent {
    /// The `new XRLayerEvent(..)` constructor, creating a new XRLayerEvent instance
    pub fn new(type_: &DOMString, event_init_dict: &Any) -> XRLayerEvent {
        Self {
            inner: Any::global("XRLayerEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRLayerEvent {
    /// Getter of the `layer` attribute.
    /// [`XRLayerEvent.layer`](https://developer.mozilla.org/en-US/docs/Web/API/XRLayerEvent/layer)
    pub fn layer(&self) -> XRLayer {
        self.inner.get("layer").as_::<XRLayer>()
    }
}
