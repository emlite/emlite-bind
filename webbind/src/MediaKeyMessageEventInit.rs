use super::*;




/// The MediaKeyMessageEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeyMessageEventInit {
    inner: Any,
}

impl FromVal for MediaKeyMessageEventInit {
    fn from_val(v: &Any) -> Self {
        MediaKeyMessageEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaKeyMessageEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaKeyMessageEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaKeyMessageEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaKeyMessageEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaKeyMessageEventInit> for Any {
    fn from(s: MediaKeyMessageEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaKeyMessageEventInit> for Any {
    fn from(s: &MediaKeyMessageEventInit) -> Any {
        s.inner.clone()
    }
}

impl MediaKeyMessageEventInit {
    /// Getter of the `messageType` attribute.
    pub fn message_type(&self) -> MediaKeyMessageType {
        self.inner.get("messageType").as_::<MediaKeyMessageType>()
    }

    /// Setter of the `messageType` attribute.
    pub fn set_message_type(&mut self, value: &MediaKeyMessageType) {
        self.inner.set("messageType", value);
    }
}
impl MediaKeyMessageEventInit {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> ArrayBuffer {
        self.inner.get("message").as_::<ArrayBuffer>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &ArrayBuffer) {
        self.inner.set("message", value);
    }
}
