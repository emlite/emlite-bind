use super::*;

/// The XRCylinderLayer class.
/// [`XRCylinderLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCylinderLayer {
    inner: XRCompositionLayer,
}
impl FromVal for XRCylinderLayer {
    fn from_val(v: &Any) -> Self {
        XRCylinderLayer {
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
impl core::ops::Deref for XRCylinderLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRCylinderLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRCylinderLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRCylinderLayer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRCylinderLayer> for Any {
    fn from(s: XRCylinderLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRCylinderLayer> for Any {
    fn from(s: &XRCylinderLayer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRCylinderLayer);

impl XRCylinderLayer {
    /// Getter of the `space` attribute.
    /// [`XRCylinderLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/space)
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    /// Setter of the `space` attribute.
    /// [`XRCylinderLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/space)
    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRCylinderLayer {
    /// Getter of the `transform` attribute.
    /// [`XRCylinderLayer.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    /// Setter of the `transform` attribute.
    /// [`XRCylinderLayer.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/transform)
    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRCylinderLayer {
    /// Getter of the `radius` attribute.
    /// [`XRCylinderLayer.radius`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/radius)
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    /// Setter of the `radius` attribute.
    /// [`XRCylinderLayer.radius`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/radius)
    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRCylinderLayer {
    /// Getter of the `centralAngle` attribute.
    /// [`XRCylinderLayer.centralAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/centralAngle)
    pub fn central_angle(&self) -> f32 {
        self.inner.get("centralAngle").as_::<f32>()
    }

    /// Setter of the `centralAngle` attribute.
    /// [`XRCylinderLayer.centralAngle`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/centralAngle)
    pub fn set_central_angle(&mut self, value: f32) {
        self.inner.set("centralAngle", value);
    }
}
impl XRCylinderLayer {
    /// Getter of the `aspectRatio` attribute.
    /// [`XRCylinderLayer.aspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/aspectRatio)
    pub fn aspect_ratio(&self) -> f32 {
        self.inner.get("aspectRatio").as_::<f32>()
    }

    /// Setter of the `aspectRatio` attribute.
    /// [`XRCylinderLayer.aspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/aspectRatio)
    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.inner.set("aspectRatio", value);
    }
}
impl XRCylinderLayer {
    /// Getter of the `onredraw` attribute.
    /// [`XRCylinderLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/onredraw)
    pub fn onredraw(&self) -> Any {
        self.inner.get("onredraw").as_::<Any>()
    }

    /// Setter of the `onredraw` attribute.
    /// [`XRCylinderLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRCylinderLayer/onredraw)
    pub fn set_onredraw(&mut self, value: &Any) {
        self.inner.set("onredraw", value);
    }
}
