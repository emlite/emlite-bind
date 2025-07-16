use super::*;

/// The XRViewerPose class.
/// [`XRViewerPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewerPose)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRViewerPose {
    inner: XRPose,
}
impl FromVal for XRViewerPose {
    fn from_val(v: &Any) -> Self {
        XRViewerPose {
            inner: XRPose::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRViewerPose {
    type Target = XRPose;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRViewerPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRViewerPose {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRViewerPose {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRViewerPose> for Any {
    fn from(s: XRViewerPose) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRViewerPose> for Any {
    fn from(s: &XRViewerPose) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRViewerPose);

impl XRViewerPose {
    /// Getter of the `views` attribute.
    /// [`XRViewerPose.views`](https://developer.mozilla.org/en-US/docs/Web/API/XRViewerPose/views)
    pub fn views(&self) -> FrozenArray<XRView> {
        self.inner.get("views").as_::<FrozenArray<XRView>>()
    }
}
