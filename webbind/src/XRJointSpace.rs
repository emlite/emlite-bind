use super::*;




/// The XRJointSpace class.
/// [`XRJointSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRJointSpace)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRJointSpace {
    inner: XRSpace,
}

impl FromVal for XRJointSpace {
    fn from_val(v: &Any) -> Self {
        XRJointSpace { inner: XRSpace::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRJointSpace {
    type Target = XRSpace;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRJointSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRJointSpace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRJointSpace {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRJointSpace> for Any {
    fn from(s: XRJointSpace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRJointSpace> for Any {
    fn from(s: &XRJointSpace) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRJointSpace);


impl XRJointSpace {
    /// Getter of the `jointName` attribute.
    /// [`XRJointSpace.jointName`](https://developer.mozilla.org/en-US/docs/Web/API/XRJointSpace/jointName)
    pub fn joint_name(&self) -> XRHandJoint {
        self.inner.get("jointName").as_::<XRHandJoint>()
    }

}
