use super::*;

/// The XRJointPose class.
/// [`XRJointPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRJointPose)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRJointPose {
    inner: XRPose,
}
impl FromVal for XRJointPose {
    fn from_val(v: &Any) -> Self {
        XRJointPose {
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
impl core::ops::Deref for XRJointPose {
    type Target = XRPose;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRJointPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRJointPose {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRJointPose {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRJointPose> for Any {
    fn from(s: XRJointPose) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRJointPose> for Any {
    fn from(s: &XRJointPose) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRJointPose);

impl XRJointPose {
    /// Getter of the `radius` attribute.
    /// [`XRJointPose.radius`](https://developer.mozilla.org/en-US/docs/Web/API/XRJointPose/radius)
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }
}
