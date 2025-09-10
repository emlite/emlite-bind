use super::*;

/// The RsaHashedKeyAlgorithm dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaHashedKeyAlgorithm {
    inner: Any,
}

impl FromVal for RsaHashedKeyAlgorithm {
    fn from_val(v: &Any) -> Self {
        RsaHashedKeyAlgorithm { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RsaHashedKeyAlgorithm {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RsaHashedKeyAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RsaHashedKeyAlgorithm {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RsaHashedKeyAlgorithm {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RsaHashedKeyAlgorithm> for Any {
    fn from(s: RsaHashedKeyAlgorithm) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RsaHashedKeyAlgorithm> for Any {
    fn from(s: &RsaHashedKeyAlgorithm) -> Any {
        s.inner.clone()
    }
}

impl RsaHashedKeyAlgorithm {
    /// Getter of the `hash` attribute.
    pub fn hash(&self) -> KeyAlgorithm {
        self.inner.get("hash").as_::<KeyAlgorithm>()
    }

    /// Setter of the `hash` attribute.
    pub fn set_hash(&mut self, value: &KeyAlgorithm) {
        self.inner.set("hash", value);
    }
}
