use super::*;

/// The BlobEvent class.
/// [`BlobEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BlobEvent {
    inner: Event,
}
impl FromVal for BlobEvent {
    fn from_val(v: &Any) -> Self {
        BlobEvent {
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
impl core::ops::Deref for BlobEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BlobEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BlobEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BlobEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BlobEvent> for Any {
    fn from(s: BlobEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BlobEvent> for Any {
    fn from(s: &BlobEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BlobEvent);

impl BlobEvent {
    /// The `new BlobEvent(..)` constructor, creating a new BlobEvent instance
    pub fn new(type_: &JsString, event_init_dict: &BlobEventInit) -> BlobEvent {
        Self {
            inner: Any::global("BlobEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl BlobEvent {
    /// Getter of the `data` attribute.
    /// [`BlobEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/data)
    pub fn data(&self) -> Blob {
        self.inner.get("data").as_::<Blob>()
    }
}
impl BlobEvent {
    /// Getter of the `timecode` attribute.
    /// [`BlobEvent.timecode`](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/timecode)
    pub fn timecode(&self) -> Any {
        self.inner.get("timecode").as_::<Any>()
    }
}
