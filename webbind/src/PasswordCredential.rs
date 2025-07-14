use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<PasswordCredential> for emlite::Val {
    fn from(s: PasswordCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PasswordCredential {
    pub fn new(data: jsbind::Any) -> PasswordCredential {
        Self {
            inner: emlite::Val::global("PasswordCredential")
                .new(&[data.into()])
                .as_::<Credential>(),
        }
    }
}
impl PasswordCredential {
    pub fn password(&self) -> jsbind::USVString {
        self.inner.get("password").as_::<jsbind::USVString>()
    }
}
impl PasswordCredential {
    pub fn name(&self) -> jsbind::USVString {
        self.inner.get("name").as_::<jsbind::USVString>()
    }
}
impl PasswordCredential {
    pub fn icon_url(&self) -> jsbind::USVString {
        self.inner.get("iconURL").as_::<jsbind::USVString>()
    }
}
