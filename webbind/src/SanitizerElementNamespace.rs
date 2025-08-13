use super::*;




/// The SanitizerElementNamespace dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SanitizerElementNamespace {
    inner: Any,
}

impl FromVal for SanitizerElementNamespace {
    fn from_val(v: &Any) -> Self {
        SanitizerElementNamespace { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SanitizerElementNamespace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SanitizerElementNamespace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SanitizerElementNamespace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SanitizerElementNamespace {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SanitizerElementNamespace> for Any {
    fn from(s: SanitizerElementNamespace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SanitizerElementNamespace> for Any {
    fn from(s: &SanitizerElementNamespace) -> Any {
        s.inner.clone()
    }
}

impl SanitizerElementNamespace {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl SanitizerElementNamespace {
    /// Getter of the `namespace` attribute.
    pub fn namespace(&self) -> JsString {
        self.inner.get("namespace").as_::<JsString>()
    }

    /// Setter of the `namespace` attribute.
    pub fn set_namespace(&mut self, value: &JsString) {
        self.inner.set("namespace", value);
    }
}
