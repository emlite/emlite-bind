use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OTPCredentialRequestOptions {
    inner: Any,
}
impl FromVal for OTPCredentialRequestOptions {
    fn from_val(v: &Any) -> Self {
        OTPCredentialRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OTPCredentialRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OTPCredentialRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OTPCredentialRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OTPCredentialRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OTPCredentialRequestOptions> for Any {
    fn from(s: OTPCredentialRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OTPCredentialRequestOptions> for Any {
    fn from(s: &OTPCredentialRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl OTPCredentialRequestOptions {
    pub fn transport(&self) -> TypedArray<OTPCredentialTransportType> {
        self.inner
            .get("transport")
            .as_::<TypedArray<OTPCredentialTransportType>>()
    }

    pub fn set_transport(&mut self, value: &TypedArray<OTPCredentialTransportType>) {
        self.inner.set("transport", value);
    }
}
