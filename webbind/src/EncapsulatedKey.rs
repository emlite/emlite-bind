use super::*;

/// The EncapsulatedKey dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncapsulatedKey {
    inner: Any,
}

impl FromVal for EncapsulatedKey {
    fn from_val(v: &Any) -> Self {
        EncapsulatedKey { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EncapsulatedKey {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncapsulatedKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncapsulatedKey {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncapsulatedKey {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EncapsulatedKey> for Any {
    fn from(s: EncapsulatedKey) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncapsulatedKey> for Any {
    fn from(s: &EncapsulatedKey) -> Any {
        s.inner.clone()
    }
}

impl EncapsulatedKey {
    /// Getter of the `sharedKey` attribute.
    pub fn shared_key(&self) -> CryptoKey {
        self.inner.get("sharedKey").as_::<CryptoKey>()
    }

    /// Setter of the `sharedKey` attribute.
    pub fn set_shared_key(&mut self, value: &CryptoKey) {
        self.inner.set("sharedKey", value);
    }
}
impl EncapsulatedKey {
    /// Getter of the `ciphertext` attribute.
    pub fn ciphertext(&self) -> ArrayBuffer {
        self.inner.get("ciphertext").as_::<ArrayBuffer>()
    }

    /// Setter of the `ciphertext` attribute.
    pub fn set_ciphertext(&mut self, value: &ArrayBuffer) {
        self.inner.set("ciphertext", value);
    }
}
