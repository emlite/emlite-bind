use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUUncapturedErrorEvent {
    inner: Event,
}
impl FromVal for GPUUncapturedErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        GPUUncapturedErrorEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUUncapturedErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUUncapturedErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUUncapturedErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUUncapturedErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GPUUncapturedErrorEvent> for emlite::Val {
    fn from(s: GPUUncapturedErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUUncapturedErrorEvent);



impl GPUUncapturedErrorEvent {
    pub fn new(type_: DOMString, gpu_uncaptured_error_event_init_dict: Any) -> GPUUncapturedErrorEvent {
        Self {
            inner: emlite::Val::global("GPUUncapturedErrorEvent").new(&[type_.into(), gpu_uncaptured_error_event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl GPUUncapturedErrorEvent {
    pub fn error(&self) -> GPUError {
        self.inner.get("error").as_::<GPUError>()
    }

}
