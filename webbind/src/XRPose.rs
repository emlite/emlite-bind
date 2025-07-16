use super::*;

/// The XRPose class.
/// [`XRPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRPose)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRPose {
    inner: Any,
}
impl FromVal for XRPose {
    fn from_val(v: &Any) -> Self {
        XRPose {
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
impl core::ops::Deref for XRPose {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRPose {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRPose {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRPose> for Any {
    fn from(s: XRPose) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRPose> for Any {
    fn from(s: &XRPose) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRPose);

impl XRPose {
    /// Getter of the `transform` attribute.
    /// [`XRPose.transform`](https://developer.mozilla.org/en-US/docs/Web/API/XRPose/transform)
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}
impl XRPose {
    /// Getter of the `linearVelocity` attribute.
    /// [`XRPose.linearVelocity`](https://developer.mozilla.org/en-US/docs/Web/API/XRPose/linearVelocity)
    pub fn linear_velocity(&self) -> DOMPointReadOnly {
        self.inner.get("linearVelocity").as_::<DOMPointReadOnly>()
    }
}
impl XRPose {
    /// Getter of the `angularVelocity` attribute.
    /// [`XRPose.angularVelocity`](https://developer.mozilla.org/en-US/docs/Web/API/XRPose/angularVelocity)
    pub fn angular_velocity(&self) -> DOMPointReadOnly {
        self.inner.get("angularVelocity").as_::<DOMPointReadOnly>()
    }
}
impl XRPose {
    /// Getter of the `emulatedPosition` attribute.
    /// [`XRPose.emulatedPosition`](https://developer.mozilla.org/en-US/docs/Web/API/XRPose/emulatedPosition)
    pub fn emulated_position(&self) -> bool {
        self.inner.get("emulatedPosition").as_::<bool>()
    }
}
