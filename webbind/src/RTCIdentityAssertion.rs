use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIdentityAssertion {
    inner: emlite::Val,
}
impl FromVal for RTCIdentityAssertion {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIdentityAssertion {
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
impl core::ops::Deref for RTCIdentityAssertion {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIdentityAssertion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIdentityAssertion {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIdentityAssertion {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCIdentityAssertion> for emlite::Val {
    fn from(s: RTCIdentityAssertion) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCIdentityAssertion);

impl RTCIdentityAssertion {
    pub fn new(idp: jsbind::DOMString, name: jsbind::DOMString) -> RTCIdentityAssertion {
        Self {
            inner: emlite::Val::global("RTCIdentityAssertion")
                .new(&[idp.into(), name.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl RTCIdentityAssertion {
    pub fn idp(&self) -> jsbind::DOMString {
        self.inner.get("idp").as_::<jsbind::DOMString>()
    }

    pub fn set_idp(&mut self, value: jsbind::DOMString) {
        self.inner.set("idp", value);
    }
}
impl RTCIdentityAssertion {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
