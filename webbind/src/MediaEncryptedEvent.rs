use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaEncryptedEvent {
    inner: Event,
}
impl FromVal for MediaEncryptedEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MediaEncryptedEvent {
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
impl AsRef<emlite::Val> for MediaEncryptedEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaEncryptedEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaEncryptedEvent> for emlite::Val {
    fn from(s: MediaEncryptedEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaEncryptedEvent> for emlite::Val {
    fn from(s: &MediaEncryptedEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaEncryptedEvent);

impl MediaEncryptedEvent {
    pub fn new0(type_: DOMString) -> MediaEncryptedEvent {
        Self {
            inner: emlite::Val::global("MediaEncryptedEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> MediaEncryptedEvent {
        Self {
            inner: emlite::Val::global("MediaEncryptedEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl MediaEncryptedEvent {
    pub fn init_data_type(&self) -> DOMString {
        self.inner.get("initDataType").as_::<DOMString>()
    }
}
impl MediaEncryptedEvent {
    pub fn init_data(&self) -> ArrayBuffer {
        self.inner.get("initData").as_::<ArrayBuffer>()
    }
}
