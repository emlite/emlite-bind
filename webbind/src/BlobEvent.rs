use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BlobEvent {
    inner: Event,
}
impl FromVal for BlobEvent {
    fn from_val(v: &emlite::Val) -> Self {
        BlobEvent {
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
impl AsRef<emlite::Val> for BlobEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BlobEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BlobEvent> for emlite::Val {
    fn from(s: BlobEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BlobEvent);

impl BlobEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> BlobEvent {
        Self {
            inner: emlite::Val::global("BlobEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl BlobEvent {
    pub fn data(&self) -> Blob {
        self.inner.get("data").as_::<Blob>()
    }
}
impl BlobEvent {
    pub fn timecode(&self) -> jsbind::Any {
        self.inner.get("timecode").as_::<jsbind::Any>()
    }
}
