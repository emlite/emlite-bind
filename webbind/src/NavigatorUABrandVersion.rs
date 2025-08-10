use super::*;

/// The NavigatorUABrandVersion dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigatorUABrandVersion {
    inner: Any,
}

impl FromVal for NavigatorUABrandVersion {
    fn from_val(v: &Any) -> Self {
        NavigatorUABrandVersion { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigatorUABrandVersion {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigatorUABrandVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigatorUABrandVersion {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigatorUABrandVersion {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigatorUABrandVersion> for Any {
    fn from(s: NavigatorUABrandVersion) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigatorUABrandVersion> for Any {
    fn from(s: &NavigatorUABrandVersion) -> Any {
        s.inner.clone()
    }
}

impl NavigatorUABrandVersion {
    /// Getter of the `brand` attribute.
    pub fn brand(&self) -> JsString {
        self.inner.get("brand").as_::<JsString>()
    }

    /// Setter of the `brand` attribute.
    pub fn set_brand(&mut self, value: &JsString) {
        self.inner.set("brand", value);
    }
}
impl NavigatorUABrandVersion {
    /// Getter of the `version` attribute.
    pub fn version(&self) -> JsString {
        self.inner.get("version").as_::<JsString>()
    }

    /// Setter of the `version` attribute.
    pub fn set_version(&mut self, value: &JsString) {
        self.inner.set("version", value);
    }
}
