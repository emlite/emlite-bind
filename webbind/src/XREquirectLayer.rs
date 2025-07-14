use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XREquirectLayer {
    inner: XRCompositionLayer,
}
impl FromVal for XREquirectLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XREquirectLayer {
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
impl From<XREquirectLayer> for emlite::Val {
    fn from(s: XREquirectLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XREquirectLayer {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: XRSpace) {
        self.inner.set("space", value);
    }
}
impl XREquirectLayer {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XREquirectLayer {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XREquirectLayer {
    pub fn central_horizontal_angle(&self) -> f32 {
        self.inner.get("centralHorizontalAngle").as_::<f32>()
    }

    pub fn set_central_horizontal_angle(&mut self, value: f32) {
        self.inner.set("centralHorizontalAngle", value);
    }
}
impl XREquirectLayer {
    pub fn upper_vertical_angle(&self) -> f32 {
        self.inner.get("upperVerticalAngle").as_::<f32>()
    }

    pub fn set_upper_vertical_angle(&mut self, value: f32) {
        self.inner.set("upperVerticalAngle", value);
    }
}
impl XREquirectLayer {
    pub fn lower_vertical_angle(&self) -> f32 {
        self.inner.get("lowerVerticalAngle").as_::<f32>()
    }

    pub fn set_lower_vertical_angle(&mut self, value: f32) {
        self.inner.set("lowerVerticalAngle", value);
    }
}
impl XREquirectLayer {
    pub fn onredraw(&self) -> jsbind::Any {
        self.inner.get("onredraw").as_::<jsbind::Any>()
    }

    pub fn set_onredraw(&mut self, value: jsbind::Any) {
        self.inner.set("onredraw", value);
    }
}
