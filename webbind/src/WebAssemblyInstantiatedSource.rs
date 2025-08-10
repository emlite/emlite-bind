use super::*;

/// The WebAssemblyInstantiatedSource dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebAssemblyInstantiatedSource {
    inner: Any,
}

impl FromVal for WebAssemblyInstantiatedSource {
    fn from_val(v: &Any) -> Self {
        WebAssemblyInstantiatedSource { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebAssemblyInstantiatedSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebAssemblyInstantiatedSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebAssemblyInstantiatedSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebAssemblyInstantiatedSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebAssemblyInstantiatedSource> for Any {
    fn from(s: WebAssemblyInstantiatedSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebAssemblyInstantiatedSource> for Any {
    fn from(s: &WebAssemblyInstantiatedSource) -> Any {
        s.inner.clone()
    }
}

impl WebAssemblyInstantiatedSource {
    /// Getter of the `module` attribute.
    pub fn module(&self) -> Module {
        self.inner.get("module").as_::<Module>()
    }

    /// Setter of the `module` attribute.
    pub fn set_module(&mut self, value: &Module) {
        self.inner.set("module", value);
    }
}
impl WebAssemblyInstantiatedSource {
    /// Getter of the `instance` attribute.
    pub fn instance(&self) -> Instance {
        self.inner.get("instance").as_::<Instance>()
    }

    /// Setter of the `instance` attribute.
    pub fn set_instance(&mut self, value: &Instance) {
        self.inner.set("instance", value);
    }
}
