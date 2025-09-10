use super::*;

/// The GPUUncapturedErrorEvent class.
/// [`GPUUncapturedErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUUncapturedErrorEvent {
    inner: Event,
}

impl FromVal for GPUUncapturedErrorEvent {
    fn from_val(v: &Any) -> Self {
        GPUUncapturedErrorEvent {
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

impl AsRef<Any> for GPUUncapturedErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUUncapturedErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUUncapturedErrorEvent> for Any {
    fn from(s: GPUUncapturedErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUUncapturedErrorEvent> for Any {
    fn from(s: &GPUUncapturedErrorEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUUncapturedErrorEvent);

impl GPUUncapturedErrorEvent {
    /// The `new GPUUncapturedErrorEvent(..)` constructor, creating a new GPUUncapturedErrorEvent instance
    pub fn new(
        type_: &JsString,
        gpu_uncaptured_error_event_init_dict: &GPUUncapturedErrorEventInit,
    ) -> GPUUncapturedErrorEvent {
        Self {
            inner: Any::global("GPUUncapturedErrorEvent")
                .new(&[type_.into(), gpu_uncaptured_error_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl GPUUncapturedErrorEvent {
    /// Getter of the `error` attribute.
    /// [`GPUUncapturedErrorEvent.error`](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent/error)
    pub fn error(&self) -> GPUError {
        self.inner.get("error").as_::<GPUError>()
    }
}
