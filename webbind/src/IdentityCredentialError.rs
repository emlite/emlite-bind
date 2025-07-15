use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityCredentialError {
    inner: DOMException,
}
impl FromVal for IdentityCredentialError {
    fn from_val(v: &emlite::Val) -> Self {
        IdentityCredentialError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for IdentityCredentialError {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdentityCredentialError {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IdentityCredentialError> for emlite::Val {
    fn from(s: IdentityCredentialError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&IdentityCredentialError> for emlite::Val {
    fn from(s: &IdentityCredentialError) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IdentityCredentialError);

impl IdentityCredentialError {
    pub fn new0() -> IdentityCredentialError {
        Self {
            inner: emlite::Val::global("IdentityCredentialError")
                .new(&[])
                .as_::<DOMException>(),
        }
    }

    pub fn new1(message: &str) -> IdentityCredentialError {
        Self {
            inner: emlite::Val::global("IdentityCredentialError")
                .new(&[message.into()])
                .as_::<DOMException>(),
        }
    }

    pub fn new2(message: &str, options: &Any) -> IdentityCredentialError {
        Self {
            inner: emlite::Val::global("IdentityCredentialError")
                .new(&[message.into(), options.into()])
                .as_::<DOMException>(),
        }
    }
}
impl IdentityCredentialError {
    pub fn error(&self) -> String {
        self.inner.get("error").as_::<String>()
    }
}
impl IdentityCredentialError {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
