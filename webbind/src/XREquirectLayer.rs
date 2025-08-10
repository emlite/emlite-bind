use super::*;

/// The XREquirectLayer class.
/// [`XREquirectLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XREquirectLayer {
    inner: XRCompositionLayer,
}

impl FromVal for XREquirectLayer {
    fn from_val(v: &Any) -> Self {
        XREquirectLayer {
            inner: XRCompositionLayer::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XREquirectLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XREquirectLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XREquirectLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XREquirectLayer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XREquirectLayer> for Any {
    fn from(s: XREquirectLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XREquirectLayer> for Any {
    fn from(s: &XREquirectLayer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XREquirectLayer);

impl XREquirectLayer {
    /// Getter of the `space` attribute.
    /// [`XREquirectLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/space)
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    /// Setter of the `space` attribute.
    /// [`XREquirectLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/space)
    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XREquirectLayer {
    /// Getter of the `transform` attribute.
    /// [`XREquirectLayer.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    /// Setter of the `transform` attribute.
    /// [`XREquirectLayer.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/transform)
    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XREquirectLayer {
    /// Getter of the `radius` attribute.
    /// [`XREquirectLayer.radius`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/radius)
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    /// Setter of the `radius` attribute.
    /// [`XREquirectLayer.radius`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/radius)
    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XREquirectLayer {
    /// Getter of the `centralHorizontalAngle` attribute.
    /// [`XREquirectLayer.centralHorizontalAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/centralHorizontalAngle)
    pub fn central_horizontal_angle(&self) -> f32 {
        self.inner.get("centralHorizontalAngle").as_::<f32>()
    }

    /// Setter of the `centralHorizontalAngle` attribute.
    /// [`XREquirectLayer.centralHorizontalAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/centralHorizontalAngle)
    pub fn set_central_horizontal_angle(&mut self, value: f32) {
        self.inner.set("centralHorizontalAngle", value);
    }
}
impl XREquirectLayer {
    /// Getter of the `upperVerticalAngle` attribute.
    /// [`XREquirectLayer.upperVerticalAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/upperVerticalAngle)
    pub fn upper_vertical_angle(&self) -> f32 {
        self.inner.get("upperVerticalAngle").as_::<f32>()
    }

    /// Setter of the `upperVerticalAngle` attribute.
    /// [`XREquirectLayer.upperVerticalAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/upperVerticalAngle)
    pub fn set_upper_vertical_angle(&mut self, value: f32) {
        self.inner.set("upperVerticalAngle", value);
    }
}
impl XREquirectLayer {
    /// Getter of the `lowerVerticalAngle` attribute.
    /// [`XREquirectLayer.lowerVerticalAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/lowerVerticalAngle)
    pub fn lower_vertical_angle(&self) -> f32 {
        self.inner.get("lowerVerticalAngle").as_::<f32>()
    }

    /// Setter of the `lowerVerticalAngle` attribute.
    /// [`XREquirectLayer.lowerVerticalAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/lowerVerticalAngle)
    pub fn set_lower_vertical_angle(&mut self, value: f32) {
        self.inner.set("lowerVerticalAngle", value);
    }
}
impl XREquirectLayer {
    /// Getter of the `onredraw` attribute.
    /// [`XREquirectLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/onredraw)
    pub fn onredraw(&self) -> Any {
        self.inner.get("onredraw").as_::<Any>()
    }

    /// Setter of the `onredraw` attribute.
    /// [`XREquirectLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XREquirectLayer/onredraw)
    pub fn set_onredraw(&mut self, value: &Any) {
        self.inner.set("onredraw", value);
    }
}
