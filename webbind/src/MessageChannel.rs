use super::*;




/// The MessageChannel class.
/// [`MessageChannel`](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MessageChannel {
    inner: Any,
}

impl FromVal for MessageChannel {
    fn from_val(v: &Any) -> Self {
        MessageChannel { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MessageChannel {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MessageChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MessageChannel {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MessageChannel {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MessageChannel> for Any {
    fn from(s: MessageChannel) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MessageChannel> for Any {
    fn from(s: &MessageChannel) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MessageChannel);



impl MessageChannel {
    /// The `new MessageChannel(..)` constructor, creating a new MessageChannel instance
    pub fn new() -> MessageChannel {
        Self {
            inner: Any::global("MessageChannel").new(&[]).as_::<Any>(),
        }
    }

}
impl MessageChannel {
    /// Getter of the `port1` attribute.
    /// [`MessageChannel.port1`](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port1)
    pub fn port1(&self) -> MessagePort {
        self.inner.get("port1").as_::<MessagePort>()
    }

}
impl MessageChannel {
    /// Getter of the `port2` attribute.
    /// [`MessageChannel.port2`](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port2)
    pub fn port2(&self) -> MessagePort {
        self.inner.get("port2").as_::<MessagePort>()
    }

}
