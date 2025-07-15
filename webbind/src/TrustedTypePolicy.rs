use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicy {
    inner: emlite::Val,
}
impl FromVal for TrustedTypePolicy {
    fn from_val(v: &emlite::Val) -> Self {
        TrustedTypePolicy {
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
impl core::ops::Deref for TrustedTypePolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedTypePolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TrustedTypePolicy {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TrustedTypePolicy {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TrustedTypePolicy> for emlite::Val {
    fn from(s: TrustedTypePolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TrustedTypePolicy);

impl TrustedTypePolicy {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl TrustedTypePolicy {
    pub fn create_html(&self, input: DOMString, arguments: Any) -> TrustedHTML {
        self.inner
            .call("createHTML", &[input.into(), arguments.into()])
            .as_::<TrustedHTML>()
    }
}
impl TrustedTypePolicy {
    pub fn create_script(&self, input: DOMString, arguments: Any) -> TrustedScript {
        self.inner
            .call("createScript", &[input.into(), arguments.into()])
            .as_::<TrustedScript>()
    }
}
impl TrustedTypePolicy {
    pub fn create_script_url(&self, input: DOMString, arguments: Any) -> TrustedScriptURL {
        self.inner
            .call("createScriptURL", &[input.into(), arguments.into()])
            .as_::<TrustedScriptURL>()
    }
}
