use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Credential {
    inner: emlite::Val,
}
impl FromVal for Credential {
    fn from_val(v: &emlite::Val) -> Self {
        Credential {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Credential {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Credential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Credential {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Credential {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Credential> for emlite::Val {
    fn from(s: Credential) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Credential);

impl Credential {
    pub fn id(&self) -> jsbind::USVString {
        self.inner.get("id").as_::<jsbind::USVString>()
    }
}
impl Credential {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }
}
impl Credential {
    pub fn is_conditional_mediation_available() -> jsbind::Promise {
        emlite::Val::global("credential")
            .call("isConditionalMediationAvailable", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Credential {
    pub fn will_request_conditional_creation() -> jsbind::Promise {
        emlite::Val::global("credential")
            .call("willRequestConditionalCreation", &[])
            .as_::<jsbind::Promise>()
    }
}
