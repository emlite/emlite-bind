use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRJointSpace {
    inner: XRSpace,
}
impl FromVal for XRJointSpace {
    fn from_val(v: &emlite::Val) -> Self {
        XRJointSpace {
            inner: XRSpace::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<XRJointSpace> for emlite::Val {
    fn from(s: XRJointSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRJointSpace {
    pub fn joint_name(&self) -> XRHandJoint {
        self.inner.get("jointName").as_::<XRHandJoint>()
    }
}
