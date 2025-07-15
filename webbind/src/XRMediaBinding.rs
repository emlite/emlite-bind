use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaQuadLayerInit {
    inner: emlite::Val,
}
impl FromVal for XRMediaQuadLayerInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRMediaQuadLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRMediaQuadLayerInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRMediaQuadLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRMediaQuadLayerInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRMediaQuadLayerInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRMediaQuadLayerInit> for emlite::Val {
    fn from(s: XRMediaQuadLayerInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRMediaQuadLayerInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRMediaQuadLayerInit {
    pub fn width(&self) -> f32 {
        self.inner.get("width").as_::<f32>()
    }

    pub fn set_width(&mut self, value: f32) {
        self.inner.set("width", value);
    }
}
impl XRMediaQuadLayerInit {
    pub fn height(&self) -> f32 {
        self.inner.get("height").as_::<f32>()
    }

    pub fn set_height(&mut self, value: f32) {
        self.inner.set("height", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaCylinderLayerInit {
    inner: emlite::Val,
}
impl FromVal for XRMediaCylinderLayerInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRMediaCylinderLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRMediaCylinderLayerInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRMediaCylinderLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRMediaCylinderLayerInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRMediaCylinderLayerInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRMediaCylinderLayerInit> for emlite::Val {
    fn from(s: XRMediaCylinderLayerInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRMediaCylinderLayerInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRMediaCylinderLayerInit {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRMediaCylinderLayerInit {
    pub fn central_angle(&self) -> f32 {
        self.inner.get("centralAngle").as_::<f32>()
    }

    pub fn set_central_angle(&mut self, value: f32) {
        self.inner.set("centralAngle", value);
    }
}
impl XRMediaCylinderLayerInit {
    pub fn aspect_ratio(&self) -> f32 {
        self.inner.get("aspectRatio").as_::<f32>()
    }

    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.inner.set("aspectRatio", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaEquirectLayerInit {
    inner: emlite::Val,
}
impl FromVal for XRMediaEquirectLayerInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRMediaEquirectLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRMediaEquirectLayerInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRMediaEquirectLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRMediaEquirectLayerInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRMediaEquirectLayerInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRMediaEquirectLayerInit> for emlite::Val {
    fn from(s: XRMediaEquirectLayerInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRMediaEquirectLayerInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRMediaEquirectLayerInit {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRMediaEquirectLayerInit {
    pub fn central_horizontal_angle(&self) -> f32 {
        self.inner.get("centralHorizontalAngle").as_::<f32>()
    }

    pub fn set_central_horizontal_angle(&mut self, value: f32) {
        self.inner.set("centralHorizontalAngle", value);
    }
}
impl XRMediaEquirectLayerInit {
    pub fn upper_vertical_angle(&self) -> f32 {
        self.inner.get("upperVerticalAngle").as_::<f32>()
    }

    pub fn set_upper_vertical_angle(&mut self, value: f32) {
        self.inner.set("upperVerticalAngle", value);
    }
}
impl XRMediaEquirectLayerInit {
    pub fn lower_vertical_angle(&self) -> f32 {
        self.inner.get("lowerVerticalAngle").as_::<f32>()
    }

    pub fn set_lower_vertical_angle(&mut self, value: f32) {
        self.inner.set("lowerVerticalAngle", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMediaBinding {
    inner: emlite::Val,
}
impl FromVal for XRMediaBinding {
    fn from_val(v: &emlite::Val) -> Self {
        XRMediaBinding {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRMediaBinding {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRMediaBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRMediaBinding {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRMediaBinding {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRMediaBinding> for emlite::Val {
    fn from(s: XRMediaBinding) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRMediaBinding);

impl XRMediaBinding {
    pub fn new(session: XRSession) -> XRMediaBinding {
        Self {
            inner: emlite::Val::global("XRMediaBinding")
                .new(&[session.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl XRMediaBinding {
    pub fn create_quad_layer0(&self, video: HTMLVideoElement) -> XRQuadLayer {
        self.inner
            .call("createQuadLayer", &[video.into()])
            .as_::<XRQuadLayer>()
    }

    pub fn create_quad_layer1(
        &self,
        video: HTMLVideoElement,
        init: XRMediaQuadLayerInit,
    ) -> XRQuadLayer {
        self.inner
            .call("createQuadLayer", &[video.into(), init.into()])
            .as_::<XRQuadLayer>()
    }
}
impl XRMediaBinding {
    pub fn create_cylinder_layer0(&self, video: HTMLVideoElement) -> XRCylinderLayer {
        self.inner
            .call("createCylinderLayer", &[video.into()])
            .as_::<XRCylinderLayer>()
    }

    pub fn create_cylinder_layer1(
        &self,
        video: HTMLVideoElement,
        init: XRMediaCylinderLayerInit,
    ) -> XRCylinderLayer {
        self.inner
            .call("createCylinderLayer", &[video.into(), init.into()])
            .as_::<XRCylinderLayer>()
    }
}
impl XRMediaBinding {
    pub fn create_equirect_layer0(&self, video: HTMLVideoElement) -> XREquirectLayer {
        self.inner
            .call("createEquirectLayer", &[video.into()])
            .as_::<XREquirectLayer>()
    }

    pub fn create_equirect_layer1(
        &self,
        video: HTMLVideoElement,
        init: XRMediaEquirectLayerInit,
    ) -> XREquirectLayer {
        self.inner
            .call("createEquirectLayer", &[video.into(), init.into()])
            .as_::<XREquirectLayer>()
    }
}
