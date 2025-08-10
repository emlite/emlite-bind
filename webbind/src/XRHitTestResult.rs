use super::*;

/// The XRHitTestResult class.
/// [`XRHitTestResult`](https://developer.mozilla.org/en-US/docs/Web/API/XRHitTestResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRHitTestResult {
    inner: Any,
}

impl FromVal for XRHitTestResult {
    fn from_val(v: &Any) -> Self {
        XRHitTestResult {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRHitTestResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRHitTestResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRHitTestResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRHitTestResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRHitTestResult> for Any {
    fn from(s: XRHitTestResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRHitTestResult> for Any {
    fn from(s: &XRHitTestResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRHitTestResult);

impl XRHitTestResult {
    /// The getPose method.
    /// [`XRHitTestResult.getPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRHitTestResult/getPose)
    pub fn get_pose(&self, base_space: &XRSpace) -> XRPose {
        self.inner
            .call("getPose", &[base_space.into()])
            .as_::<XRPose>()
    }
}
impl XRHitTestResult {
    /// The createAnchor method.
    /// [`XRHitTestResult.createAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/XRHitTestResult/createAnchor)
    pub fn create_anchor(&self) -> Promise<XRAnchor> {
        self.inner
            .call("createAnchor", &[])
            .as_::<Promise<XRAnchor>>()
    }
}
