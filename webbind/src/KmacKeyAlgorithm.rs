use super::*;

/// The KmacKeyAlgorithm dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KmacKeyAlgorithm {
    inner: Any,
}

impl FromVal for KmacKeyAlgorithm {
    fn from_val(v: &Any) -> Self {
        KmacKeyAlgorithm { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KmacKeyAlgorithm {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KmacKeyAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KmacKeyAlgorithm {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KmacKeyAlgorithm {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KmacKeyAlgorithm> for Any {
    fn from(s: KmacKeyAlgorithm) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KmacKeyAlgorithm> for Any {
    fn from(s: &KmacKeyAlgorithm) -> Any {
        s.inner.clone()
    }
}

impl KmacKeyAlgorithm {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
