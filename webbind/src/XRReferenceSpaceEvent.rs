use super::*;

/// The XRReferenceSpaceEvent class.
/// [`XRReferenceSpaceEvent`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRReferenceSpaceEvent {
    inner: Event,
}

impl FromVal for XRReferenceSpaceEvent {
    fn from_val(v: &Any) -> Self {
        XRReferenceSpaceEvent {
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

impl core::ops::Deref for XRReferenceSpaceEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRReferenceSpaceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRReferenceSpaceEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRReferenceSpaceEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRReferenceSpaceEvent> for Any {
    fn from(s: XRReferenceSpaceEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRReferenceSpaceEvent> for Any {
    fn from(s: &XRReferenceSpaceEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRReferenceSpaceEvent);

impl XRReferenceSpaceEvent {
    /// Getter of the `referenceSpace` attribute.
    /// [`XRReferenceSpaceEvent.referenceSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/referenceSpace)
    pub fn reference_space(&self) -> XRReferenceSpace {
        self.inner.get("referenceSpace").as_::<XRReferenceSpace>()
    }
}
impl XRReferenceSpaceEvent {
    /// Getter of the `transform` attribute.
    /// [`XRReferenceSpaceEvent.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}

impl XRReferenceSpaceEvent {
    /// The `new XRReferenceSpaceEvent(..)` constructor, creating a new XRReferenceSpaceEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &XRReferenceSpaceEventInit,
    ) -> XRReferenceSpaceEvent {
        Self {
            inner: Any::global("XRReferenceSpaceEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
