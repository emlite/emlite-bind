use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SFrameTransformErrorEvent {
    inner: Event,
}
impl FromVal for SFrameTransformErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SFrameTransformErrorEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for SFrameTransformErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SFrameTransformErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SFrameTransformErrorEvent> for emlite::Val {
    fn from(s: SFrameTransformErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SFrameTransformErrorEvent> for emlite::Val {
    fn from(s: &SFrameTransformErrorEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SFrameTransformErrorEvent);

impl SFrameTransformErrorEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> SFrameTransformErrorEvent {
        Self {
            inner: emlite::Val::global("SFrameTransformErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SFrameTransformErrorEvent {
    pub fn error_type(&self) -> SFrameTransformErrorEventType {
        self.inner
            .get("errorType")
            .as_::<SFrameTransformErrorEventType>()
    }
}
impl SFrameTransformErrorEvent {
    pub fn key_id(&self) -> Any {
        self.inner.get("keyID").as_::<Any>()
    }
}
impl SFrameTransformErrorEvent {
    pub fn frame(&self) -> Any {
        self.inner.get("frame").as_::<Any>()
    }
}
