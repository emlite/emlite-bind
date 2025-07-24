use super::*;

/// The OTPCredential class.
/// [`OTPCredential`](https://developer.mozilla.org/en-US/docs/Web/API/OTPCredential)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OTPCredential {
    inner: Credential,
}
impl FromVal for OTPCredential {
    fn from_val(v: &Any) -> Self {
        OTPCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OTPCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OTPCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OTPCredential {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OTPCredential {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OTPCredential> for Any {
    fn from(s: OTPCredential) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OTPCredential> for Any {
    fn from(s: &OTPCredential) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OTPCredential);

impl OTPCredential {
    /// Getter of the `code` attribute.
    /// [`OTPCredential.code`](https://developer.mozilla.org/en-US/docs/Web/API/OTPCredential/code)
    pub fn code(&self) -> DOMString {
        self.inner.get("code").as_::<DOMString>()
    }
}
