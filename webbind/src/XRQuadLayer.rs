use super::*;

/// The XRQuadLayer class.
/// [`XRQuadLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRQuadLayer {
    inner: XRCompositionLayer,
}

impl FromVal for XRQuadLayer {
    fn from_val(v: &Any) -> Self {
        XRQuadLayer {
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

impl core::ops::Deref for XRQuadLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRQuadLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRQuadLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRQuadLayer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRQuadLayer> for Any {
    fn from(s: XRQuadLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRQuadLayer> for Any {
    fn from(s: &XRQuadLayer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRQuadLayer);

impl XRQuadLayer {
    /// Getter of the `space` attribute.
    /// [`XRQuadLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/space)
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    /// Setter of the `space` attribute.
    /// [`XRQuadLayer.space`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/space)
    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRQuadLayer {
    /// Getter of the `transform` attribute.
    /// [`XRQuadLayer.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    /// Setter of the `transform` attribute.
    /// [`XRQuadLayer.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/transform)
    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRQuadLayer {
    /// Getter of the `width` attribute.
    /// [`XRQuadLayer.width`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/width)
    pub fn width(&self) -> f32 {
        self.inner.get("width").as_::<f32>()
    }

    /// Setter of the `width` attribute.
    /// [`XRQuadLayer.width`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/width)
    pub fn set_width(&mut self, value: f32) {
        self.inner.set("width", value);
    }
}
impl XRQuadLayer {
    /// Getter of the `height` attribute.
    /// [`XRQuadLayer.height`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/height)
    pub fn height(&self) -> f32 {
        self.inner.get("height").as_::<f32>()
    }

    /// Setter of the `height` attribute.
    /// [`XRQuadLayer.height`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/height)
    pub fn set_height(&mut self, value: f32) {
        self.inner.set("height", value);
    }
}
impl XRQuadLayer {
    /// Getter of the `onredraw` attribute.
    /// [`XRQuadLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/onredraw)
    pub fn onredraw(&self) -> Any {
        self.inner.get("onredraw").as_::<Any>()
    }

    /// Setter of the `onredraw` attribute.
    /// [`XRQuadLayer.onredraw`](https://developer.mozilla.org/en-US/docs/Web/API/XRQuadLayer/onredraw)
    pub fn set_onredraw(&mut self, value: &Any) {
        self.inner.set("onredraw", value);
    }
}
