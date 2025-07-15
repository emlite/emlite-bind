use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRProjectionLayer {
    inner: XRCompositionLayer,
}
impl FromVal for XRProjectionLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRProjectionLayer {
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
impl AsRef<emlite::Val> for XRProjectionLayer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRProjectionLayer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRProjectionLayer> for emlite::Val {
    fn from(s: XRProjectionLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRProjectionLayer> for emlite::Val {
    fn from(s: &XRProjectionLayer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRProjectionLayer);

impl XRProjectionLayer {
    pub fn texture_width(&self) -> u32 {
        self.inner.get("textureWidth").as_::<u32>()
    }
}
impl XRProjectionLayer {
    pub fn texture_height(&self) -> u32 {
        self.inner.get("textureHeight").as_::<u32>()
    }
}
impl XRProjectionLayer {
    pub fn texture_array_length(&self) -> u32 {
        self.inner.get("textureArrayLength").as_::<u32>()
    }
}
impl XRProjectionLayer {
    pub fn ignore_depth_values(&self) -> bool {
        self.inner.get("ignoreDepthValues").as_::<bool>()
    }
}
impl XRProjectionLayer {
    pub fn fixed_foveation(&self) -> f32 {
        self.inner.get("fixedFoveation").as_::<f32>()
    }

    pub fn set_fixed_foveation(&mut self, value: f32) {
        self.inner.set("fixedFoveation", value);
    }
}
impl XRProjectionLayer {
    pub fn delta_pose(&self) -> XRRigidTransform {
        self.inner.get("deltaPose").as_::<XRRigidTransform>()
    }

    pub fn set_delta_pose(&mut self, value: &XRRigidTransform) {
        self.inner.set("deltaPose", value);
    }
}
