use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRPose {
    inner: emlite::Val,
}
impl FromVal for XRPose {
    fn from_val(v: &emlite::Val) -> Self {
        XRPose {
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
impl core::ops::Deref for XRPose {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRPose> for emlite::Val {
    fn from(s: XRPose) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRPose {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}
impl XRPose {
    pub fn linear_velocity(&self) -> DOMPointReadOnly {
        self.inner.get("linearVelocity").as_::<DOMPointReadOnly>()
    }
}
impl XRPose {
    pub fn angular_velocity(&self) -> DOMPointReadOnly {
        self.inner.get("angularVelocity").as_::<DOMPointReadOnly>()
    }
}
impl XRPose {
    pub fn emulated_position(&self) -> bool {
        self.inner.get("emulatedPosition").as_::<bool>()
    }
}
