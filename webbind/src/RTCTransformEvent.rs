use super::*;

/// The RTCTransformEvent class.
/// [`RTCTransformEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTransformEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCTransformEvent {
    inner: Event,
}
impl FromVal for RTCTransformEvent {
    fn from_val(v: &Any) -> Self {
        RTCTransformEvent {
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
impl core::ops::Deref for RTCTransformEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCTransformEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCTransformEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCTransformEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCTransformEvent> for Any {
    fn from(s: RTCTransformEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCTransformEvent> for Any {
    fn from(s: &RTCTransformEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCTransformEvent);

impl RTCTransformEvent {
    /// Getter of the `transformer` attribute.
    /// [`RTCTransformEvent.transformer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTransformEvent/transformer)
    pub fn transformer(&self) -> RTCRtpScriptTransformer {
        self.inner
            .get("transformer")
            .as_::<RTCRtpScriptTransformer>()
    }
}
