use super::*;




/// The LockInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LockInfo {
    inner: Any,
}

impl FromVal for LockInfo {
    fn from_val(v: &Any) -> Self {
        LockInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LockInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LockInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LockInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LockInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LockInfo> for Any {
    fn from(s: LockInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LockInfo> for Any {
    fn from(s: &LockInfo) -> Any {
        s.inner.clone()
    }
}

impl LockInfo {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl LockInfo {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> LockMode {
        self.inner.get("mode").as_::<LockMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &LockMode) {
        self.inner.set("mode", value);
    }
}
impl LockInfo {
    /// Getter of the `clientId` attribute.
    pub fn client_id(&self) -> JsString {
        self.inner.get("clientId").as_::<JsString>()
    }

    /// Setter of the `clientId` attribute.
    pub fn set_client_id(&mut self, value: &JsString) {
        self.inner.set("clientId", value);
    }
}
