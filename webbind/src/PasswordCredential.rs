use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PasswordCredential {
    inner: Credential,
}
impl FromVal for PasswordCredential {
    fn from_val(v: &emlite::Val) -> Self {
        PasswordCredential {
            inner: Credential::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PasswordCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PasswordCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PasswordCredential {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PasswordCredential {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PasswordCredential> for emlite::Val {
    fn from(s: PasswordCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PasswordCredential);

impl PasswordCredential {
    pub fn new(data: Any) -> PasswordCredential {
        Self {
            inner: emlite::Val::global("PasswordCredential")
                .new(&[data.into()])
                .as_::<Credential>(),
        }
    }
}
impl PasswordCredential {
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }
}
impl PasswordCredential {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }
}
impl PasswordCredential {
    pub fn icon_url(&self) -> USVString {
        self.inner.get("iconURL").as_::<USVString>()
    }
}
