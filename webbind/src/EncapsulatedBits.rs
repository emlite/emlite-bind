use super::*;

/// The EncapsulatedBits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EncapsulatedBits {
    inner: Any,
}

impl FromVal for EncapsulatedBits {
    fn from_val(v: &Any) -> Self {
        EncapsulatedBits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EncapsulatedBits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EncapsulatedBits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EncapsulatedBits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EncapsulatedBits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EncapsulatedBits> for Any {
    fn from(s: EncapsulatedBits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EncapsulatedBits> for Any {
    fn from(s: &EncapsulatedBits) -> Any {
        s.inner.clone()
    }
}

impl EncapsulatedBits {
    /// Getter of the `sharedKey` attribute.
    pub fn shared_key(&self) -> ArrayBuffer {
        self.inner.get("sharedKey").as_::<ArrayBuffer>()
    }

    /// Setter of the `sharedKey` attribute.
    pub fn set_shared_key(&mut self, value: &ArrayBuffer) {
        self.inner.set("sharedKey", value);
    }
}
impl EncapsulatedBits {
    /// Getter of the `ciphertext` attribute.
    pub fn ciphertext(&self) -> ArrayBuffer {
        self.inner.get("ciphertext").as_::<ArrayBuffer>()
    }

    /// Setter of the `ciphertext` attribute.
    pub fn set_ciphertext(&mut self, value: &ArrayBuffer) {
        self.inner.set("ciphertext", value);
    }
}
