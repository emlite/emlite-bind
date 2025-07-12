use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for TrustedTypePolicy {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TrustedTypePolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TrustedTypePolicy> for emlite::Val {
    fn from(s: TrustedTypePolicy) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TrustedTypePolicy {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl TrustedTypePolicy {
    pub fn create_html(&self, input: jsbind::DOMString, arguments: jsbind::Any) -> TrustedHTML {
        self.inner
            .call("createHTML", &[input.into(), arguments.into()])
            .as_::<TrustedHTML>()
    }
}
impl TrustedTypePolicy {
    pub fn create_script(&self, input: jsbind::DOMString, arguments: jsbind::Any) -> TrustedScript {
        self.inner
            .call("createScript", &[input.into(), arguments.into()])
            .as_::<TrustedScript>()
    }
}
impl TrustedTypePolicy {
    pub fn create_script_url(
        &self,
        input: jsbind::DOMString,
        arguments: jsbind::Any,
    ) -> TrustedScriptURL {
        self.inner
            .call("createScriptURL", &[input.into(), arguments.into()])
            .as_::<TrustedScriptURL>()
    }
}
