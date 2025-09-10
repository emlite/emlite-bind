use super::*;

/// The IdentityCredentialError class.
/// [`IdentityCredentialError`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredentialError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialError {
    inner: DOMException,
}

impl FromVal for IdentityCredentialError {
    fn from_val(v: &Any) -> Self {
        IdentityCredentialError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityCredentialError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityCredentialError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityCredentialError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityCredentialError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IdentityCredentialError> for Any {
    fn from(s: IdentityCredentialError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityCredentialError> for Any {
    fn from(s: &IdentityCredentialError) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IdentityCredentialError);

impl IdentityCredentialError {
    /// The `new IdentityCredentialError(..)` constructor, creating a new IdentityCredentialError instance
    pub fn new0() -> IdentityCredentialError {
        Self {
            inner: Any::global("IdentityCredentialError")
                .new(&[])
                .as_::<DOMException>(),
        }
    }

    /// The `new IdentityCredentialError(..)` constructor, creating a new IdentityCredentialError instance
    pub fn new1(message: &JsString) -> IdentityCredentialError {
        Self {
            inner: Any::global("IdentityCredentialError")
                .new(&[message.into()])
                .as_::<DOMException>(),
        }
    }

    /// The `new IdentityCredentialError(..)` constructor, creating a new IdentityCredentialError instance
    pub fn new2(
        message: &JsString,
        options: &IdentityCredentialErrorInit,
    ) -> IdentityCredentialError {
        Self {
            inner: Any::global("IdentityCredentialError")
                .new(&[message.into(), options.into()])
                .as_::<DOMException>(),
        }
    }
}
impl IdentityCredentialError {
    /// Getter of the `error` attribute.
    /// [`IdentityCredentialError.error`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredentialError/error)
    pub fn error(&self) -> JsString {
        self.inner.get("error").as_::<JsString>()
    }
}
impl IdentityCredentialError {
    /// Getter of the `url` attribute.
    /// [`IdentityCredentialError.url`](https://developer.mozilla.org/en-US/docs/Web/API/IdentityCredentialError/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
