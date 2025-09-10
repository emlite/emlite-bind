use super::*;

/// The XRInputSourceEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSourceEventInit {
    inner: Any,
}

impl FromVal for XRInputSourceEventInit {
    fn from_val(v: &Any) -> Self {
        XRInputSourceEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRInputSourceEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRInputSourceEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRInputSourceEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRInputSourceEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRInputSourceEventInit> for Any {
    fn from(s: XRInputSourceEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRInputSourceEventInit> for Any {
    fn from(s: &XRInputSourceEventInit) -> Any {
        s.inner.clone()
    }
}

impl XRInputSourceEventInit {
    /// Getter of the `frame` attribute.
    pub fn frame(&self) -> XRFrame {
        self.inner.get("frame").as_::<XRFrame>()
    }

    /// Setter of the `frame` attribute.
    pub fn set_frame(&mut self, value: &XRFrame) {
        self.inner.set("frame", value);
    }
}
impl XRInputSourceEventInit {
    /// Getter of the `inputSource` attribute.
    pub fn input_source(&self) -> XRInputSource {
        self.inner.get("inputSource").as_::<XRInputSource>()
    }

    /// Setter of the `inputSource` attribute.
    pub fn set_input_source(&mut self, value: &XRInputSource) {
        self.inner.set("inputSource", value);
    }
}
