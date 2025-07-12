use super::*;

#[derive(Clone, Debug)]
pub struct XRQuadLayer {
    inner: XRCompositionLayer,
}
impl FromVal for XRQuadLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRQuadLayer {
            inner: XRCompositionLayer::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRQuadLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRQuadLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRQuadLayer> for emlite::Val {
    fn from(s: XRQuadLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRQuadLayer {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRQuadLayer {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRQuadLayer {
    pub fn width(&self) -> f32 {
        self.inner.get("width").as_::<f32>()
    }

    pub fn set_width(&mut self, value: f32) {
        self.inner.set("width", value);
    }
}
impl XRQuadLayer {
    pub fn height(&self) -> f32 {
        self.inner.get("height").as_::<f32>()
    }

    pub fn set_height(&mut self, value: f32) {
        self.inner.set("height", value);
    }
}
impl XRQuadLayer {
    pub fn onredraw(&self) -> jsbind::Any {
        self.inner.get("onredraw").as_::<jsbind::Any>()
    }

    pub fn set_onredraw(&mut self, value: jsbind::Any) {
        self.inner.set("onredraw", value);
    }
}
