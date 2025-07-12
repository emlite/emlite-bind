use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for XRLightProbe {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRLightProbe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRLightProbe> for emlite::Val {
    fn from(s: XRLightProbe) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRLightProbe {
    pub fn probe_space(&self) -> XRSpace {
        self.inner.get("probeSpace").as_::<XRSpace>()
    }
}
impl XRLightProbe {
    pub fn onreflectionchange(&self) -> jsbind::Any {
        self.inner.get("onreflectionchange").as_::<jsbind::Any>()
    }

    pub fn set_onreflectionchange(&mut self, value: jsbind::Any) {
        self.inner.set("onreflectionchange", value);
    }
}
