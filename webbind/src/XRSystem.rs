use super::*;




/// The XRSystem class.
/// [`XRSystem`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSystem {
    inner: EventTarget,
}

impl FromVal for XRSystem {
    fn from_val(v: &Any) -> Self {
        XRSystem { inner: EventTarget::from_val(v) }
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
        self.inner.call("isSessionSupported", &[mode.into(), ]).as_::<Promise<bool>>()
    }
}
impl XRSystem {
    /// The requestSession method.
    /// [`XRSystem.requestSession`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)
    pub fn request_session0(&self, mode: &XRSessionMode) -> Promise<XRSession> {
        self.inner.call("requestSession", &[mode.into(), ]).as_::<Promise<XRSession>>()
    }
    /// The requestSession method.
    /// [`XRSystem.requestSession`](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)
    pub fn request_session1(&self, mode: &XRSessionMode, options: &XRSessionInit) -> Promise<XRSession> {
        self.inner.call("requestSession", &[mode.into(), options.into(), ]).as_::<Promise<XRSession>>()
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
