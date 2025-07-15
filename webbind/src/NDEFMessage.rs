use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFMessage {
    inner: emlite::Val,
}
impl FromVal for NDEFMessage {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFMessage { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFMessage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NDEFMessage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFMessage {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<NDEFMessage> for emlite::Val {
    fn from(s: NDEFMessage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NDEFMessage);



impl NDEFMessage {
    pub fn new(message_init: Any) -> NDEFMessage {
        Self {
            inner: emlite::Val::global("NDEFMessage").new(&[message_init.into()]).as_::<emlite::Val>(),
        }
    }

}
impl NDEFMessage {
    pub fn records(&self) -> FrozenArray<NDEFRecord> {
        self.inner.get("records").as_::<FrozenArray<NDEFRecord>>()
    }

}
