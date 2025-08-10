use super::*;

/// The AesGcmParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AesGcmParams {
    inner: Any,
}

impl FromVal for AesGcmParams {
    fn from_val(v: &Any) -> Self {
        AesGcmParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AesGcmParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AesGcmParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AesGcmParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AesGcmParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AesGcmParams> for Any {
    fn from(s: AesGcmParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AesGcmParams> for Any {
    fn from(s: &AesGcmParams) -> Any {
        s.inner.clone()
    }
}

impl AesGcmParams {
    /// Getter of the `iv` attribute.
    pub fn iv(&self) -> Any {
        self.inner.get("iv").as_::<Any>()
    }

    /// Setter of the `iv` attribute.
    pub fn set_iv(&mut self, value: &Any) {
        self.inner.set("iv", value);
    }
}
impl AesGcmParams {
    /// Getter of the `additionalData` attribute.
    pub fn additional_data(&self) -> Any {
        self.inner.get("additionalData").as_::<Any>()
    }

    /// Setter of the `additionalData` attribute.
    pub fn set_additional_data(&mut self, value: &Any) {
        self.inner.set("additionalData", value);
    }
}
impl AesGcmParams {
    /// Getter of the `tagLength` attribute.
    pub fn tag_length(&self) -> u8 {
        self.inner.get("tagLength").as_::<u8>()
    }

    /// Setter of the `tagLength` attribute.
    pub fn set_tag_length(&mut self, value: u8) {
        self.inner.set("tagLength", value);
    }
}
