use super::*;

/// The CaptureHandleConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureHandleConfig {
    inner: Any,
}

impl FromVal for CaptureHandleConfig {
    fn from_val(v: &Any) -> Self {
        CaptureHandleConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CaptureHandleConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CaptureHandleConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CaptureHandleConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CaptureHandleConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CaptureHandleConfig> for Any {
    fn from(s: CaptureHandleConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CaptureHandleConfig> for Any {
    fn from(s: &CaptureHandleConfig) -> Any {
        s.inner.clone()
    }
}

impl CaptureHandleConfig {
    /// Getter of the `exposeOrigin` attribute.
    pub fn expose_origin(&self) -> bool {
        self.inner.get("exposeOrigin").as_::<bool>()
    }

    /// Setter of the `exposeOrigin` attribute.
    pub fn set_expose_origin(&mut self, value: bool) {
        self.inner.set("exposeOrigin", value);
    }
}
impl CaptureHandleConfig {
    /// Getter of the `handle` attribute.
    pub fn handle(&self) -> JsString {
        self.inner.get("handle").as_::<JsString>()
    }

    /// Setter of the `handle` attribute.
    pub fn set_handle(&mut self, value: &JsString) {
        self.inner.set("handle", value);
    }
}
impl CaptureHandleConfig {
    /// Getter of the `permittedOrigins` attribute.
    pub fn permitted_origins(&self) -> TypedArray<JsString> {
        self.inner
            .get("permittedOrigins")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `permittedOrigins` attribute.
    pub fn set_permitted_origins(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("permittedOrigins", value);
    }
}
