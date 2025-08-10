use super::*;

/// The SharedStorageUrlWithMetadata dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageUrlWithMetadata {
    inner: Any,
}

impl FromVal for SharedStorageUrlWithMetadata {
    fn from_val(v: &Any) -> Self {
        SharedStorageUrlWithMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SharedStorageUrlWithMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedStorageUrlWithMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedStorageUrlWithMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedStorageUrlWithMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SharedStorageUrlWithMetadata> for Any {
    fn from(s: SharedStorageUrlWithMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedStorageUrlWithMetadata> for Any {
    fn from(s: &SharedStorageUrlWithMetadata) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageUrlWithMetadata {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl SharedStorageUrlWithMetadata {
    /// Getter of the `reportingMetadata` attribute.
    pub fn reporting_metadata(&self) -> Object {
        self.inner.get("reportingMetadata").as_::<Object>()
    }

    /// Setter of the `reportingMetadata` attribute.
    pub fn set_reporting_metadata(&mut self, value: &Object) {
        self.inner.set("reportingMetadata", value);
    }
}
