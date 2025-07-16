use super::*;

/// The XRLightProbe class.
/// [`XRLightProbe`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightProbe)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLightProbe {
    inner: EventTarget,
}
impl FromVal for XRLightProbe {
    fn from_val(v: &Any) -> Self {
        XRLightProbe {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for XRLightProbe {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRLightProbe {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRLightProbe> for Any {
    fn from(s: XRLightProbe) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRLightProbe> for Any {
    fn from(s: &XRLightProbe) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRLightProbe);

impl XRLightProbe {
    /// Getter of the `probeSpace` attribute.
    /// [`XRLightProbe.probeSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightProbe/probeSpace)
    pub fn probe_space(&self) -> XRSpace {
        self.inner.get("probeSpace").as_::<XRSpace>()
    }
}
impl XRLightProbe {
    /// Getter of the `onreflectionchange` attribute.
    /// [`XRLightProbe.onreflectionchange`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightProbe/onreflectionchange)
    pub fn onreflectionchange(&self) -> Any {
        self.inner.get("onreflectionchange").as_::<Any>()
    }

    /// Setter of the `onreflectionchange` attribute.
    /// [`XRLightProbe.onreflectionchange`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightProbe/onreflectionchange)
    pub fn set_onreflectionchange(&mut self, value: &Any) {
        self.inner.set("onreflectionchange", value);
    }
}
