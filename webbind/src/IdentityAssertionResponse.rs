use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityAssertionResponse {
    inner: Any,
}
impl FromVal for IdentityAssertionResponse {
    fn from_val(v: &Any) -> Self {
        IdentityAssertionResponse { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdentityAssertionResponse {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdentityAssertionResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IdentityAssertionResponse {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IdentityAssertionResponse {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IdentityAssertionResponse> for Any {
    fn from(s: IdentityAssertionResponse) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IdentityAssertionResponse> for Any {
    fn from(s: &IdentityAssertionResponse) -> Any {
        s.inner.clone()
    }
}

impl IdentityAssertionResponse {
    pub fn token(&self) -> JsString {
        self.inner.get("token").as_::<JsString>()
    }

    pub fn set_token(&mut self, value: &JsString) {
        self.inner.set("token", value);
    }
}
impl IdentityAssertionResponse {
    pub fn continue_on(&self) -> JsString {
        self.inner.get("continue_on").as_::<JsString>()
    }

    pub fn set_continue_on(&mut self, value: &JsString) {
        self.inner.set("continue_on", value);
    }
}
impl IdentityAssertionResponse {
    pub fn error(&self) -> IdentityCredentialErrorInit {
        self.inner.get("error").as_::<IdentityCredentialErrorInit>()
    }

    pub fn set_error(&mut self, value: &IdentityCredentialErrorInit) {
        self.inner.set("error", value);
    }
}
