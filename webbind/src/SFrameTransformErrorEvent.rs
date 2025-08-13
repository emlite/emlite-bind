use super::*;




/// The SFrameTransformErrorEvent class.
/// [`SFrameTransformErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SFrameTransformErrorEvent {
    inner: Event,
}

impl FromVal for SFrameTransformErrorEvent {
    fn from_val(v: &Any) -> Self {
        SFrameTransformErrorEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SFrameTransformErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SFrameTransformErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SFrameTransformErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SFrameTransformErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SFrameTransformErrorEvent> for Any {
    fn from(s: SFrameTransformErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SFrameTransformErrorEvent> for Any {
    fn from(s: &SFrameTransformErrorEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SFrameTransformErrorEvent);



impl SFrameTransformErrorEvent {
    /// The `new SFrameTransformErrorEvent(..)` constructor, creating a new SFrameTransformErrorEvent instance
    pub fn new(type_: &JsString, event_init_dict: &SFrameTransformErrorEventInit) -> SFrameTransformErrorEvent {
        Self {
            inner: Any::global("SFrameTransformErrorEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl SFrameTransformErrorEvent {
    /// Getter of the `errorType` attribute.
    /// [`SFrameTransformErrorEvent.errorType`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/errorType)
    pub fn error_type(&self) -> SFrameTransformErrorEventType {
        self.inner.get("errorType").as_::<SFrameTransformErrorEventType>()
    }

}
impl SFrameTransformErrorEvent {
    /// Getter of the `keyID` attribute.
    /// [`SFrameTransformErrorEvent.keyID`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/keyID)
    pub fn key_id(&self) -> Any {
        self.inner.get("keyID").as_::<Any>()
    }

}
impl SFrameTransformErrorEvent {
    /// Getter of the `frame` attribute.
    /// [`SFrameTransformErrorEvent.frame`](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/frame)
    pub fn frame(&self) -> Any {
        self.inner.get("frame").as_::<Any>()
    }

}
