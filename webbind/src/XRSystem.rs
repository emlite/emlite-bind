use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSessionInit {
    inner: Any,
}
impl FromVal for XRSessionInit {
    fn from_val(v: &Any) -> Self {
        XRSessionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSessionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSessionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRSessionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRSessionInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRSessionInit> for Any {
    fn from(s: XRSessionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRSessionInit> for Any {
    fn from(s: &XRSessionInit) -> Any {
        s.inner.clone()
    }
}

impl XRSessionInit {
    pub fn required_features(&self) -> TypedArray<JsString> {
        self.inner
            .get("requiredFeatures")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_required_features(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl XRSessionInit {
    pub fn optional_features(&self) -> TypedArray<JsString> {
        self.inner
            .get("optionalFeatures")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_optional_features(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("optionalFeatures", value);
    }
}
/// The XRSystem class.
/// [`XRSystem`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSystem {
    inner: EventTarget,
}
impl FromVal for XRSystem {
    fn from_val(v: &Any) -> Self {
        XRSystem {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSystem {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRSystem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRSystem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRSystem> for Any {
    fn from(s: XRSystem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRSystem> for Any {
    fn from(s: &XRSystem) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRSystem);

impl XRSystem {
    /// The isSessionSupported method.
    /// [`XRSystem.isSessionSupported`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/isSessionSupported)
    pub fn is_session_supported(&self, mode: &XRSessionMode) -> Promise<bool> {
        self.inner
            .call("isSessionSupported", &[mode.into()])
            .as_::<Promise<bool>>()
    }
}
impl XRSystem {
    /// The requestSession method.
    /// [`XRSystem.requestSession`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)
    pub fn request_session0(&self, mode: &XRSessionMode) -> Promise<XRSession> {
        self.inner
            .call("requestSession", &[mode.into()])
            .as_::<Promise<XRSession>>()
    }
    /// The requestSession method.
    /// [`XRSystem.requestSession`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)
    pub fn request_session1(
        &self,
        mode: &XRSessionMode,
        options: &XRSessionInit,
    ) -> Promise<XRSession> {
        self.inner
            .call("requestSession", &[mode.into(), options.into()])
            .as_::<Promise<XRSession>>()
    }
}
impl XRSystem {
    /// Getter of the `ondevicechange` attribute.
    /// [`XRSystem.ondevicechange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/ondevicechange)
    pub fn ondevicechange(&self) -> Any {
        self.inner.get("ondevicechange").as_::<Any>()
    }

    /// Setter of the `ondevicechange` attribute.
    /// [`XRSystem.ondevicechange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/ondevicechange)
    pub fn set_ondevicechange(&mut self, value: &Any) {
        self.inner.set("ondevicechange", value);
    }
}
