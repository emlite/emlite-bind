use super::*;

#[derive(Clone, Debug)]
pub struct XRCylinderLayer {
    inner: XRCompositionLayer,
}
impl FromVal for XRCylinderLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRCylinderLayer {
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
impl std::ops::Deref for XRCylinderLayer {
    type Target = XRCompositionLayer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRCylinderLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRCylinderLayer> for emlite::Val {
    fn from(s: XRCylinderLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRCylinderLayer {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRCylinderLayer {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRCylinderLayer {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRCylinderLayer {
    pub fn central_angle(&self) -> f32 {
        self.inner.get("centralAngle").as_::<f32>()
    }

    pub fn set_central_angle(&mut self, value: f32) {
        self.inner.set("centralAngle", value);
    }
}
impl XRCylinderLayer {
    pub fn aspect_ratio(&self) -> f32 {
        self.inner.get("aspectRatio").as_::<f32>()
    }

    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.inner.set("aspectRatio", value);
    }
}
impl XRCylinderLayer {
    pub fn onredraw(&self) -> jsbind::Any {
        self.inner.get("onredraw").as_::<jsbind::Any>()
    }

    pub fn set_onredraw(&mut self, value: jsbind::Any) {
        self.inner.set("onredraw", value);
    }
}
