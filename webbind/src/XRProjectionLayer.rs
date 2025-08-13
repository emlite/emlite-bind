use super::*;




/// The XRProjectionLayer class.
/// [`XRProjectionLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRProjectionLayer {
    inner: XRCompositionLayer,
}

impl FromVal for XRProjectionLayer {
    fn from_val(v: &Any) -> Self {
        XRProjectionLayer { inner: XRCompositionLayer::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRProjectionLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRProjectionLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRProjectionLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRProjectionLayer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRProjectionLayer> for Any {
    fn from(s: XRProjectionLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRProjectionLayer> for Any {
    fn from(s: &XRProjectionLayer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRProjectionLayer);


impl XRProjectionLayer {
    /// Getter of the `textureWidth` attribute.
    /// [`XRProjectionLayer.textureWidth`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/textureWidth)
    pub fn texture_width(&self) -> u32 {
        self.inner.get("textureWidth").as_::<u32>()
    }

}
impl XRProjectionLayer {
    /// Getter of the `textureHeight` attribute.
    /// [`XRProjectionLayer.textureHeight`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/textureHeight)
    pub fn texture_height(&self) -> u32 {
        self.inner.get("textureHeight").as_::<u32>()
    }

}
impl XRProjectionLayer {
    /// Getter of the `textureArrayLength` attribute.
    /// [`XRProjectionLayer.textureArrayLength`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/textureArrayLength)
    pub fn texture_array_length(&self) -> u32 {
        self.inner.get("textureArrayLength").as_::<u32>()
    }

}
impl XRProjectionLayer {
    /// Getter of the `ignoreDepthValues` attribute.
    /// [`XRProjectionLayer.ignoreDepthValues`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/ignoreDepthValues)
    pub fn ignore_depth_values(&self) -> bool {
        self.inner.get("ignoreDepthValues").as_::<bool>()
    }

}
impl XRProjectionLayer {
    /// Getter of the `fixedFoveation` attribute.
    /// [`XRProjectionLayer.fixedFoveation`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/fixedFoveation)
    pub fn fixed_foveation(&self) -> f32 {
        self.inner.get("fixedFoveation").as_::<f32>()
    }

    /// Setter of the `fixedFoveation` attribute.
    /// [`XRProjectionLayer.fixedFoveation`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/fixedFoveation)
    pub fn set_fixed_foveation(&mut self, value: f32) {
        self.inner.set("fixedFoveation", value);
    }
}
impl XRProjectionLayer {
    /// Getter of the `deltaPose` attribute.
    /// [`XRProjectionLayer.deltaPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/deltaPose)
    pub fn delta_pose(&self) -> XRRigidTransform {
        self.inner.get("deltaPose").as_::<XRRigidTransform>()
    }

    /// Setter of the `deltaPose` attribute.
    /// [`XRProjectionLayer.deltaPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRProjectionLayer/deltaPose)
    pub fn set_delta_pose(&mut self, value: &XRRigidTransform) {
        self.inner.set("deltaPose", value);
    }
}
