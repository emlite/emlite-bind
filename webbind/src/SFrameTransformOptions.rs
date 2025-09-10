use super::*;

/// The SFrameTransformOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SFrameTransformOptions {
    inner: Any,
}

impl FromVal for SFrameTransformOptions {
    fn from_val(v: &Any) -> Self {
        SFrameTransformOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SFrameTransformOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SFrameTransformOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SFrameTransformOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SFrameTransformOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SFrameTransformOptions> for Any {
    fn from(s: SFrameTransformOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SFrameTransformOptions> for Any {
    fn from(s: &SFrameTransformOptions) -> Any {
        s.inner.clone()
    }
}

impl SFrameTransformOptions {
    /// Getter of the `role` attribute.
    pub fn role(&self) -> SFrameTransformRole {
        self.inner.get("role").as_::<SFrameTransformRole>()
    }

    /// Setter of the `role` attribute.
    pub fn set_role(&mut self, value: &SFrameTransformRole) {
        self.inner.set("role", value);
    }
}
impl SFrameTransformOptions {
    /// Getter of the `cipherSuite` attribute.
    pub fn cipher_suite(&self) -> SFrameCipherSuite {
        self.inner.get("cipherSuite").as_::<SFrameCipherSuite>()
    }

    /// Setter of the `cipherSuite` attribute.
    pub fn set_cipher_suite(&mut self, value: &SFrameCipherSuite) {
        self.inner.set("cipherSuite", value);
    }
}
