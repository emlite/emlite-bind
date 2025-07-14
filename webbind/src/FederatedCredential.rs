use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FederatedCredential {
    inner: Credential,
}
impl FromVal for FederatedCredential {
    fn from_val(v: &emlite::Val) -> Self {
        FederatedCredential {
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
impl core::ops::Deref for FederatedCredential {
    type Target = Credential;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FederatedCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FederatedCredential> for emlite::Val {
    fn from(s: FederatedCredential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FederatedCredential {
    pub fn new(data: jsbind::Any) -> FederatedCredential {
        Self {
            inner: emlite::Val::global("FederatedCredential")
                .new(&[data.into()])
                .as_::<Credential>(),
        }
    }
}
impl FederatedCredential {
    pub fn provider(&self) -> jsbind::USVString {
        self.inner.get("provider").as_::<jsbind::USVString>()
    }
}
impl FederatedCredential {
    pub fn protocol(&self) -> jsbind::DOMString {
        self.inner.get("protocol").as_::<jsbind::DOMString>()
    }
}
impl FederatedCredential {
    pub fn name(&self) -> jsbind::USVString {
        self.inner.get("name").as_::<jsbind::USVString>()
    }
}
impl FederatedCredential {
    pub fn icon_url(&self) -> jsbind::USVString {
        self.inner.get("iconURL").as_::<jsbind::USVString>()
    }
}
