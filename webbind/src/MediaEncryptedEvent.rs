use super::*;




/// The MediaEncryptedEvent class.
/// [`MediaEncryptedEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaEncryptedEvent {
    inner: Event,
}

impl FromVal for MediaEncryptedEvent {
    fn from_val(v: &Any) -> Self {
        MediaEncryptedEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaEncryptedEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaEncryptedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaEncryptedEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaEncryptedEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaEncryptedEvent> for Any {
    fn from(s: MediaEncryptedEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaEncryptedEvent> for Any {
    fn from(s: &MediaEncryptedEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaEncryptedEvent);



impl MediaEncryptedEvent {
    /// The `new MediaEncryptedEvent(..)` constructor, creating a new MediaEncryptedEvent instance
    pub fn new0(type_: &JsString) -> MediaEncryptedEvent {
        Self {
            inner: Any::global("MediaEncryptedEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    /// The `new MediaEncryptedEvent(..)` constructor, creating a new MediaEncryptedEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &MediaEncryptedEventInit) -> MediaEncryptedEvent {
        Self {
            inner: Any::global("MediaEncryptedEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl MediaEncryptedEvent {
    /// Getter of the `initDataType` attribute.
    /// [`MediaEncryptedEvent.initDataType`](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initDataType)
    pub fn init_data_type(&self) -> JsString {
        self.inner.get("initDataType").as_::<JsString>()
    }

}
impl MediaEncryptedEvent {
    /// Getter of the `initData` attribute.
    /// [`MediaEncryptedEvent.initData`](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initData)
    pub fn init_data(&self) -> ArrayBuffer {
        self.inner.get("initData").as_::<ArrayBuffer>()
    }

}
