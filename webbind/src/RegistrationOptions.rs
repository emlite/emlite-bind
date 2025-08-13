use super::*;




/// The RegistrationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RegistrationOptions {
    inner: Any,
}

impl FromVal for RegistrationOptions {
    fn from_val(v: &Any) -> Self {
        RegistrationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RegistrationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RegistrationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RegistrationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RegistrationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RegistrationOptions> for Any {
    fn from(s: RegistrationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RegistrationOptions> for Any {
    fn from(s: &RegistrationOptions) -> Any {
        s.inner.clone()
    }
}

impl RegistrationOptions {
    /// Getter of the `scope` attribute.
    pub fn scope(&self) -> JsString {
        self.inner.get("scope").as_::<JsString>()
    }

    /// Setter of the `scope` attribute.
    pub fn set_scope(&mut self, value: &JsString) {
        self.inner.set("scope", value);
    }
}
impl RegistrationOptions {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> WorkerType {
        self.inner.get("type").as_::<WorkerType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &WorkerType) {
        self.inner.set("type", value);
    }
}
impl RegistrationOptions {
    /// Getter of the `updateViaCache` attribute.
    pub fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        self.inner.get("updateViaCache").as_::<ServiceWorkerUpdateViaCache>()
    }

    /// Setter of the `updateViaCache` attribute.
    pub fn set_update_via_cache(&mut self, value: &ServiceWorkerUpdateViaCache) {
        self.inner.set("updateViaCache", value);
    }
}
