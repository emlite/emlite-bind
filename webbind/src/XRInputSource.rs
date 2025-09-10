use super::*;

/// The XRInputSource class.
/// [`XRInputSource`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRInputSource {
    inner: Any,
}

impl FromVal for XRInputSource {
    fn from_val(v: &Any) -> Self {
        XRInputSource {
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

impl core::ops::Deref for XRInputSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRInputSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRInputSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRInputSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRInputSource> for Any {
    fn from(s: XRInputSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRInputSource> for Any {
    fn from(s: &XRInputSource) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRInputSource);

impl XRInputSource {
    /// Getter of the `handedness` attribute.
    /// [`XRInputSource.handedness`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/handedness)
    pub fn handedness(&self) -> XRHandedness {
        self.inner.get("handedness").as_::<XRHandedness>()
    }
}
impl XRInputSource {
    /// Getter of the `targetRayMode` attribute.
    /// [`XRInputSource.targetRayMode`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/targetRayMode)
    pub fn target_ray_mode(&self) -> XRTargetRayMode {
        self.inner.get("targetRayMode").as_::<XRTargetRayMode>()
    }
}
impl XRInputSource {
    /// Getter of the `targetRaySpace` attribute.
    /// [`XRInputSource.targetRaySpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/targetRaySpace)
    pub fn target_ray_space(&self) -> XRSpace {
        self.inner.get("targetRaySpace").as_::<XRSpace>()
    }
}
impl XRInputSource {
    /// Getter of the `gripSpace` attribute.
    /// [`XRInputSource.gripSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/gripSpace)
    pub fn grip_space(&self) -> XRSpace {
        self.inner.get("gripSpace").as_::<XRSpace>()
    }
}
impl XRInputSource {
    /// Getter of the `profiles` attribute.
    /// [`XRInputSource.profiles`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/profiles)
    pub fn profiles(&self) -> TypedArray<JsString> {
        self.inner.get("profiles").as_::<TypedArray<JsString>>()
    }
}
impl XRInputSource {
    /// Getter of the `skipRendering` attribute.
    /// [`XRInputSource.skipRendering`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/skipRendering)
    pub fn skip_rendering(&self) -> bool {
        self.inner.get("skipRendering").as_::<bool>()
    }
}
impl XRInputSource {
    /// Getter of the `gamepad` attribute.
    /// [`XRInputSource.gamepad`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/gamepad)
    pub fn gamepad(&self) -> Gamepad {
        self.inner.get("gamepad").as_::<Gamepad>()
    }
}
impl XRInputSource {
    /// Getter of the `hand` attribute.
    /// [`XRInputSource.hand`](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/hand)
    pub fn hand(&self) -> XRHand {
        self.inner.get("hand").as_::<XRHand>()
    }
}
