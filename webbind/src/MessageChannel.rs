use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MessageChannel {
    inner: emlite::Val,
}
impl FromVal for MessageChannel {
    fn from_val(v: &emlite::Val) -> Self {
        MessageChannel { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MessageChannel {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MessageChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MessageChannel {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MessageChannel {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<MessageChannel> for emlite::Val {
    fn from(s: MessageChannel) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MessageChannel);



impl MessageChannel {
    pub fn new() -> MessageChannel {
        Self {
            inner: emlite::Val::global("MessageChannel").new(&[]).as_::<emlite::Val>(),
        }
    }

}
impl MessageChannel {
    pub fn port1(&self) -> Any {
        self.inner.get("port1").as_::<Any>()
    }

}
impl MessageChannel {
    pub fn port2(&self) -> Any {
        self.inner.get("port2").as_::<Any>()
    }

}
