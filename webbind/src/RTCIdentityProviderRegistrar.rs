use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProvider {
    inner: emlite::Val,
}
impl FromVal for RTCIdentityProvider {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityProvider { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIdentityProvider {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIdentityProvider {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIdentityProvider {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<RTCIdentityProvider> for emlite::Val {
    fn from(s: RTCIdentityProvider) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCIdentityProvider {
    pub fn generate_assertion(&self) -> Function {
        self.inner.get("generateAssertion").as_::<Function>()
    }

    pub fn set_generate_assertion(&mut self, value: Function) {
        self.inner.set("generateAssertion", value);
    }

}
impl RTCIdentityProvider {
    pub fn validate_assertion(&self) -> Function {
        self.inner.get("validateAssertion").as_::<Function>()
    }

    pub fn set_validate_assertion(&mut self, value: Function) {
        self.inner.set("validateAssertion", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityProviderRegistrar {
    inner: emlite::Val,
}
impl FromVal for RTCIdentityProviderRegistrar {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityProviderRegistrar { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIdentityProviderRegistrar {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityProviderRegistrar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIdentityProviderRegistrar {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIdentityProviderRegistrar {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<RTCIdentityProviderRegistrar> for emlite::Val {
    fn from(s: RTCIdentityProviderRegistrar) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCIdentityProviderRegistrar);


impl RTCIdentityProviderRegistrar {
    pub fn register(&self, idp: RTCIdentityProvider) -> Undefined {
        self.inner.call("register", &[idp.into(), ]).as_::<Undefined>()
    }

}
