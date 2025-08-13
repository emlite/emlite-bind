use super::*;




/// The XRMediaLayerInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaLayerInit {
    inner: Any,
}

impl FromVal for XRMediaLayerInit {
    fn from_val(v: &Any) -> Self {
        XRMediaLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRMediaLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRMediaLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRMediaLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRMediaLayerInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRMediaLayerInit> for Any {
    fn from(s: XRMediaLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRMediaLayerInit> for Any {
    fn from(s: &XRMediaLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRMediaLayerInit {
    /// Getter of the `space` attribute.
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    /// Setter of the `space` attribute.
    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRMediaLayerInit {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> XRLayerLayout {
        self.inner.get("layout").as_::<XRLayerLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &XRLayerLayout) {
        self.inner.set("layout", value);
    }
}
impl XRMediaLayerInit {
    /// Getter of the `invertStereo` attribute.
    pub fn invert_stereo(&self) -> bool {
        self.inner.get("invertStereo").as_::<bool>()
    }

    /// Setter of the `invertStereo` attribute.
    pub fn set_invert_stereo(&mut self, value: bool) {
        self.inner.set("invertStereo", value);
    }
}
