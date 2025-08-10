use super::*;

/// The AesKeyGenParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AesKeyGenParams {
    inner: Any,
}

impl FromVal for AesKeyGenParams {
    fn from_val(v: &Any) -> Self {
        AesKeyGenParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AesKeyGenParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AesKeyGenParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AesKeyGenParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AesKeyGenParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AesKeyGenParams> for Any {
    fn from(s: AesKeyGenParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AesKeyGenParams> for Any {
    fn from(s: &AesKeyGenParams) -> Any {
        s.inner.clone()
    }
}

impl AesKeyGenParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u16 {
        self.inner.get("length").as_::<u16>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u16) {
        self.inner.set("length", value);
    }
}
