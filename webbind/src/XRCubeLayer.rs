use super::*;




/// The XRCubeLayer class.
/// [`XRCubeLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCubeLayer {
    inner: XRCompositionLayer,
}

impl FromVal for XRCubeLayer {
    fn from_val(v: &Any) -> Self {
        XRCubeLayer { inner: XRCompositionLayer::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRCubeLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRCubeLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRCubeLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRCubeLayer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRCubeLayer> for Any {
    fn from(s: XRCubeLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRCubeLayer> for Any {
    fn from(s: &XRCubeLayer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRCubeLayer);


impl XRCubeLayer {
    /// Getter of the `space` attribute.
    /// [`XRCubeLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer/space)
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    /// Setter of the `space` attribute.
    /// [`XRCubeLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer/space)
    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRCubeLayer {
    /// Getter of the `orientation` attribute.
    /// [`XRCubeLayer.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer/orientation)
    pub fn orientation(&self) -> DOMPointReadOnly {
        self.inner.get("orientation").as_::<DOMPointReadOnly>()
    }

    /// Setter of the `orientation` attribute.
    /// [`XRCubeLayer.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer/orientation)
    pub fn set_orientation(&mut self, value: &DOMPointReadOnly) {
        self.inner.set("orientation", value);
    }
}
impl XRCubeLayer {
    /// Getter of the `onredraw` attribute.
    /// [`XRCubeLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer/onredraw)
    pub fn onredraw(&self) -> Any {
        self.inner.get("onredraw").as_::<Any>()
    }

    /// Setter of the `onredraw` attribute.
    /// [`XRCubeLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRCubeLayer/onredraw)
    pub fn set_onredraw(&mut self, value: &Any) {
        self.inner.set("onredraw", value);
    }
}
