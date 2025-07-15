use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDepthInformation {
    inner: emlite::Val,
}
impl FromVal for XRDepthInformation {
    fn from_val(v: &emlite::Val) -> Self {
        XRDepthInformation {
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
impl core::ops::Deref for XRDepthInformation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRDepthInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRDepthInformation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRDepthInformation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRDepthInformation> for emlite::Val {
    fn from(s: XRDepthInformation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRDepthInformation> for emlite::Val {
    fn from(s: &XRDepthInformation) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRDepthInformation);

impl XRDepthInformation {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }
}
impl XRDepthInformation {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }
}
impl XRDepthInformation {
    pub fn norm_depth_buffer_from_norm_view(&self) -> XRRigidTransform {
        self.inner
            .get("normDepthBufferFromNormView")
            .as_::<XRRigidTransform>()
    }
}
impl XRDepthInformation {
    pub fn raw_value_to_meters(&self) -> f32 {
        self.inner.get("rawValueToMeters").as_::<f32>()
    }
}
impl XRDepthInformation {
    pub fn projection_matrix(&self) -> Float32Array {
        self.inner.get("projectionMatrix").as_::<Float32Array>()
    }
}
impl XRDepthInformation {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}
