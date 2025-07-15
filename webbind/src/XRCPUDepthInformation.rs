use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCPUDepthInformation {
    inner: XRDepthInformation,
}
impl FromVal for XRCPUDepthInformation {
    fn from_val(v: &emlite::Val) -> Self {
        XRCPUDepthInformation { inner: XRDepthInformation::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRCPUDepthInformation {
    type Target = XRDepthInformation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRCPUDepthInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRCPUDepthInformation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRCPUDepthInformation {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRCPUDepthInformation> for emlite::Val {
    fn from(s: XRCPUDepthInformation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRCPUDepthInformation);


impl XRCPUDepthInformation {
    pub fn data(&self) -> ArrayBuffer {
        self.inner.get("data").as_::<ArrayBuffer>()
    }

}
impl XRCPUDepthInformation {
    pub fn get_depth_in_meters(&self, x: f32, y: f32) -> f32 {
        self.inner.call("getDepthInMeters", &[x.into(), y.into(), ]).as_::<f32>()
    }

}
