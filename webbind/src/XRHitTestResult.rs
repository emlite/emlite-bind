use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRHitTestResult {
    inner: emlite::Val,
}
impl FromVal for XRHitTestResult {
    fn from_val(v: &emlite::Val) -> Self {
        XRHitTestResult {
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
impl core::ops::Deref for XRHitTestResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRHitTestResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRHitTestResult> for emlite::Val {
    fn from(s: XRHitTestResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRHitTestResult {
    pub fn get_pose(&self, base_space: XRSpace) -> XRPose {
        self.inner
            .call("getPose", &[base_space.into()])
            .as_::<XRPose>()
    }
}
impl XRHitTestResult {
    pub fn create_anchor(&self) -> jsbind::Promise {
        self.inner
            .call("createAnchor", &[])
            .as_::<jsbind::Promise>()
    }
}
