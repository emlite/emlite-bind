use super::*;

/// The XRDepthInformation class.
/// [`XRDepthInformation`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDepthInformation {
    inner: Any,
}
impl FromVal for XRDepthInformation {
    fn from_val(v: &Any) -> Self {
        XRDepthInformation {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRDepthInformation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRDepthInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRDepthInformation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRDepthInformation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRDepthInformation> for Any {
    fn from(s: XRDepthInformation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRDepthInformation> for Any {
    fn from(s: &XRDepthInformation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRDepthInformation);

impl XRDepthInformation {
    /// Getter of the `width` attribute.
    /// [`XRDepthInformation.width`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }
}
impl XRDepthInformation {
    /// Getter of the `height` attribute.
    /// [`XRDepthInformation.height`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }
}
impl XRDepthInformation {
    /// Getter of the `normDepthBufferFromNormView` attribute.
    /// [`XRDepthInformation.normDepthBufferFromNormView`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation/normDepthBufferFromNormView)
    pub fn norm_depth_buffer_from_norm_view(&self) -> XRRigidTransform {
        self.inner
            .get("normDepthBufferFromNormView")
            .as_::<XRRigidTransform>()
    }
}
impl XRDepthInformation {
    /// Getter of the `rawValueToMeters` attribute.
    /// [`XRDepthInformation.rawValueToMeters`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation/rawValueToMeters)
    pub fn raw_value_to_meters(&self) -> f32 {
        self.inner.get("rawValueToMeters").as_::<f32>()
    }
}
impl XRDepthInformation {
    /// Getter of the `projectionMatrix` attribute.
    /// [`XRDepthInformation.projectionMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation/projectionMatrix)
    pub fn projection_matrix(&self) -> Float32Array {
        self.inner.get("projectionMatrix").as_::<Float32Array>()
    }
}
impl XRDepthInformation {
    /// Getter of the `transform` attribute.
    /// [`XRDepthInformation.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRDepthInformation/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}
