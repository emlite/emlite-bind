use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HashChangeEvent {
    inner: Event,
}
impl FromVal for HashChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        HashChangeEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HashChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HashChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HashChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HashChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HashChangeEvent> for emlite::Val {
    fn from(s: HashChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HashChangeEvent);



impl HashChangeEvent {
    pub fn new0(type_: DOMString) -> HashChangeEvent {
        Self {
            inner: emlite::Val::global("HashChangeEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> HashChangeEvent {
        Self {
            inner: emlite::Val::global("HashChangeEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl HashChangeEvent {
    pub fn old_url(&self) -> USVString {
        self.inner.get("oldURL").as_::<USVString>()
    }

}
impl HashChangeEvent {
    pub fn new_url(&self) -> USVString {
        self.inner.get("newURL").as_::<USVString>()
    }

}
