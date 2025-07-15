use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLightProbe {
    inner: EventTarget,
}
impl FromVal for XRLightProbe {
    fn from_val(v: &emlite::Val) -> Self {
        XRLightProbe {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLightProbe {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLightProbe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRLightProbe {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRLightProbe {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRLightProbe> for emlite::Val {
    fn from(s: XRLightProbe) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRLightProbe);

impl XRLightProbe {
    pub fn probe_space(&self) -> XRSpace {
        self.inner.get("probeSpace").as_::<XRSpace>()
    }
}
impl XRLightProbe {
    pub fn onreflectionchange(&self) -> Any {
        self.inner.get("onreflectionchange").as_::<Any>()
    }

    pub fn set_onreflectionchange(&mut self, value: Any) {
        self.inner.set("onreflectionchange", value);
    }
}
