use super::*;

/// The KmacKeyGenParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KmacKeyGenParams {
    inner: Any,
}

impl FromVal for KmacKeyGenParams {
    fn from_val(v: &Any) -> Self {
        KmacKeyGenParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KmacKeyGenParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KmacKeyGenParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KmacKeyGenParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KmacKeyGenParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KmacKeyGenParams> for Any {
    fn from(s: KmacKeyGenParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KmacKeyGenParams> for Any {
    fn from(s: &KmacKeyGenParams) -> Any {
        s.inner.clone()
    }
}

impl KmacKeyGenParams {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
