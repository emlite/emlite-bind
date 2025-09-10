use super::*;

/// The EcdhKeyDeriveParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EcdhKeyDeriveParams {
    inner: Any,
}

impl FromVal for EcdhKeyDeriveParams {
    fn from_val(v: &Any) -> Self {
        EcdhKeyDeriveParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EcdhKeyDeriveParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EcdhKeyDeriveParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EcdhKeyDeriveParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EcdhKeyDeriveParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EcdhKeyDeriveParams> for Any {
    fn from(s: EcdhKeyDeriveParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EcdhKeyDeriveParams> for Any {
    fn from(s: &EcdhKeyDeriveParams) -> Any {
        s.inner.clone()
    }
}

impl EcdhKeyDeriveParams {
    /// Getter of the `public` attribute.
    pub fn public(&self) -> CryptoKey {
        self.inner.get("public").as_::<CryptoKey>()
    }

    /// Setter of the `public` attribute.
    pub fn set_public(&mut self, value: &CryptoKey) {
        self.inner.set("public", value);
    }
}
