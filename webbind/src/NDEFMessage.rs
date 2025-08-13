use super::*;




/// The NDEFMessage class.
/// [`NDEFMessage`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFMessage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFMessage {
    inner: Any,
}

impl FromVal for NDEFMessage {
    fn from_val(v: &Any) -> Self {
        NDEFMessage { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NDEFMessage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NDEFMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NDEFMessage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NDEFMessage {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NDEFMessage> for Any {
    fn from(s: NDEFMessage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NDEFMessage> for Any {
    fn from(s: &NDEFMessage) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NDEFMessage);



impl NDEFMessage {
    /// The `new NDEFMessage(..)` constructor, creating a new NDEFMessage instance
    pub fn new(message_init: &NDEFMessageInit) -> NDEFMessage {
        Self {
            inner: Any::global("NDEFMessage").new(&[message_init.into()]).as_::<Any>(),
        }
    }

}
impl NDEFMessage {
    /// Getter of the `records` attribute.
    /// [`NDEFMessage.records`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFMessage/records)
    pub fn records(&self) -> TypedArray<NDEFRecord> {
        self.inner.get("records").as_::<TypedArray<NDEFRecord>>()
    }

}
