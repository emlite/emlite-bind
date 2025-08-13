use super::*;




/// The XRCPUDepthInformation class.
/// [`XRCPUDepthInformation`](https://developer.mozilla.org/en-US/docs/Web/API/XRCPUDepthInformation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCPUDepthInformation {
    inner: XRDepthInformation,
}

impl FromVal for XRCPUDepthInformation {
    fn from_val(v: &Any) -> Self {
        XRCPUDepthInformation { inner: XRDepthInformation::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for XRCPUDepthInformation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRCPUDepthInformation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRCPUDepthInformation> for Any {
    fn from(s: XRCPUDepthInformation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRCPUDepthInformation> for Any {
    fn from(s: &XRCPUDepthInformation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRCPUDepthInformation);


impl XRCPUDepthInformation {
    /// Getter of the `data` attribute.
    /// [`XRCPUDepthInformation.data`](https://developer.mozilla.org/en-US/docs/Web/API/XRCPUDepthInformation/data)
    pub fn data(&self) -> ArrayBuffer {
        self.inner.get("data").as_::<ArrayBuffer>()
    }

}
impl XRCPUDepthInformation {
    /// The getDepthInMeters method.
    /// [`XRCPUDepthInformation.getDepthInMeters`](https://developer.mozilla.org/en-US/docs/Web/API/XRCPUDepthInformation/getDepthInMeters)
    pub fn get_depth_in_meters(&self, x: f32, y: f32) -> f32 {
        self.inner.call("getDepthInMeters", &[x.into(), y.into(), ]).as_::<f32>()
    }
}
