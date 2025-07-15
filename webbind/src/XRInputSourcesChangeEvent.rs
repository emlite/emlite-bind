use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourcesChangeEvent {
    inner: Event,
}
impl FromVal for XRInputSourcesChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        XRInputSourcesChangeEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRInputSourcesChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRInputSourcesChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRInputSourcesChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRInputSourcesChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRInputSourcesChangeEvent> for emlite::Val {
    fn from(s: XRInputSourcesChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRInputSourcesChangeEvent);



impl XRInputSourcesChangeEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> XRInputSourcesChangeEvent {
        Self {
            inner: emlite::Val::global("XRInputSourcesChangeEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl XRInputSourcesChangeEvent {
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }

}
impl XRInputSourcesChangeEvent {
    pub fn added(&self) -> FrozenArray<XRInputSource> {
        self.inner.get("added").as_::<FrozenArray<XRInputSource>>()
    }

}
impl XRInputSourcesChangeEvent {
    pub fn removed(&self) -> FrozenArray<XRInputSource> {
        self.inner.get("removed").as_::<FrozenArray<XRInputSource>>()
    }

}
