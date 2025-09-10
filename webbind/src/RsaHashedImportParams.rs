use super::*;

/// The RsaHashedImportParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaHashedImportParams {
    inner: Any,
}

impl FromVal for RsaHashedImportParams {
    fn from_val(v: &Any) -> Self {
        RsaHashedImportParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RsaHashedImportParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RsaHashedImportParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RsaHashedImportParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RsaHashedImportParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RsaHashedImportParams> for Any {
    fn from(s: RsaHashedImportParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RsaHashedImportParams> for Any {
    fn from(s: &RsaHashedImportParams) -> Any {
        s.inner.clone()
    }
}

impl RsaHashedImportParams {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
