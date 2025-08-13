use super::*;




/// The XRCubeLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCubeLayerInit {
    inner: Any,
}

impl FromVal for XRCubeLayerInit {
    fn from_val(v: &Any) -> Self {
        XRCubeLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRCubeLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRCubeLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRCubeLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRCubeLayerInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRCubeLayerInit> for Any {
    fn from(s: XRCubeLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRCubeLayerInit> for Any {
    fn from(s: &XRCubeLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRCubeLayerInit {
    /// Getter of the `orientation` attribute.
    pub fn orientation(&self) -> DOMPointReadOnly {
        self.inner.get("orientation").as_::<DOMPointReadOnly>()
    }

    /// Setter of the `orientation` attribute.
    pub fn set_orientation(&mut self, value: &DOMPointReadOnly) {
        self.inner.set("orientation", value);
    }
}
