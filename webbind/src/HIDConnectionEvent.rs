use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDConnectionEvent {
    inner: Event,
}
impl FromVal for HIDConnectionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        HIDConnectionEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HIDConnectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDConnectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HIDConnectionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HIDConnectionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HIDConnectionEvent> for emlite::Val {
    fn from(s: HIDConnectionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HIDConnectionEvent);



impl HIDConnectionEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> HIDConnectionEvent {
        Self {
            inner: emlite::Val::global("HIDConnectionEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl HIDConnectionEvent {
    pub fn device(&self) -> HIDDevice {
        self.inner.get("device").as_::<HIDDevice>()
    }

}
