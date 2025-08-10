use super::*;

/// The SanitizerAttributeNamespace dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SanitizerAttributeNamespace {
    inner: Any,
}

impl FromVal for SanitizerAttributeNamespace {
    fn from_val(v: &Any) -> Self {
        SanitizerAttributeNamespace { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SanitizerAttributeNamespace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SanitizerAttributeNamespace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SanitizerAttributeNamespace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SanitizerAttributeNamespace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SanitizerAttributeNamespace> for Any {
    fn from(s: SanitizerAttributeNamespace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SanitizerAttributeNamespace> for Any {
    fn from(s: &SanitizerAttributeNamespace) -> Any {
        s.inner.clone()
    }
}

impl SanitizerAttributeNamespace {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl SanitizerAttributeNamespace {
    /// Getter of the `namespace` attribute.
    pub fn namespace(&self) -> JsString {
        self.inner.get("namespace").as_::<JsString>()
    }

    /// Setter of the `namespace` attribute.
    pub fn set_namespace(&mut self, value: &JsString) {
        self.inner.set("namespace", value);
    }
}
