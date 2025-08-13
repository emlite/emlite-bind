use super::*;




/// The WebTransportHash dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportHash {
    inner: Any,
}

impl FromVal for WebTransportHash {
    fn from_val(v: &Any) -> Self {
        WebTransportHash { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebTransportHash {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebTransportHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebTransportHash {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebTransportHash {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WebTransportHash> for Any {
    fn from(s: WebTransportHash) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebTransportHash> for Any {
    fn from(s: &WebTransportHash) -> Any {
        s.inner.clone()
    }
}

impl WebTransportHash {
    /// Getter of the `algorithm` attribute.
    pub fn algorithm(&self) -> JsString {
        self.inner.get("algorithm").as_::<JsString>()
    }

    /// Setter of the `algorithm` attribute.
    pub fn set_algorithm(&mut self, value: &JsString) {
        self.inner.set("algorithm", value);
    }
}
impl WebTransportHash {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
