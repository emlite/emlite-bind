use super::*;

/// The XRLightProbeInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLightProbeInit {
    inner: Any,
}

impl FromVal for XRLightProbeInit {
    fn from_val(v: &Any) -> Self {
        XRLightProbeInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRLightProbeInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRLightProbeInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRLightProbeInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRLightProbeInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRLightProbeInit> for Any {
    fn from(s: XRLightProbeInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRLightProbeInit> for Any {
    fn from(s: &XRLightProbeInit) -> Any {
        s.inner.clone()
    }
}

impl XRLightProbeInit {
    /// Getter of the `reflectionFormat` attribute.
    pub fn reflection_format(&self) -> XRReflectionFormat {
        self.inner
            .get("reflectionFormat")
            .as_::<XRReflectionFormat>()
    }

    /// Setter of the `reflectionFormat` attribute.
    pub fn set_reflection_format(&mut self, value: &XRReflectionFormat) {
        self.inner.set("reflectionFormat", value);
    }
}
