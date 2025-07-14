use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRJointPose {
    inner: XRPose,
}
impl FromVal for XRJointPose {
    fn from_val(v: &emlite::Val) -> Self {
        XRJointPose {
            inner: XRPose::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<XRJointPose> for emlite::Val {
    fn from(s: XRJointPose) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRJointPose {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }
}
