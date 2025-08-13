use super::*;




/// The PushManager class.
/// [`PushManager`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushManager {
    inner: Any,
}

impl FromVal for PushManager {
    fn from_val(v: &Any) -> Self {
        PushManager { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PushManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushManager {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PushManager> for Any {
    fn from(s: PushManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushManager> for Any {
    fn from(s: &PushManager) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PushManager);


impl PushManager {
    /// Getter of the `supportedContentEncodings` static attribute.
    /// [`PushManager.supportedContentEncodings`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/supportedContentEncodings)
    pub fn supported_content_encodings() -> TypedArray<JsString> {
        Any::global("PushManager").get("supportedContentEncodings").as_::<TypedArray<JsString>>()
    }

}
impl PushManager {
    /// The subscribe method.
    /// [`PushManager.subscribe`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)
    pub fn subscribe0(&self, ) -> Promise<PushSubscription> {
        self.inner.call("subscribe", &[]).as_::<Promise<PushSubscription>>()
    }
    /// The subscribe method.
    /// [`PushManager.subscribe`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)
    pub fn subscribe1(&self, options: &PushSubscriptionOptionsInit) -> Promise<PushSubscription> {
        self.inner.call("subscribe", &[options.into(), ]).as_::<Promise<PushSubscription>>()
    }
}
impl PushManager {
    /// The getSubscription method.
    /// [`PushManager.getSubscription`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/getSubscription)
    pub fn get_subscription(&self, ) -> Promise<PushSubscription> {
        self.inner.call("getSubscription", &[]).as_::<Promise<PushSubscription>>()
    }
}
impl PushManager {
    /// The permissionState method.
    /// [`PushManager.permissionState`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)
    pub fn permission_state0(&self, ) -> Promise<PermissionState> {
        self.inner.call("permissionState", &[]).as_::<Promise<PermissionState>>()
    }
    /// The permissionState method.
    /// [`PushManager.permissionState`](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)
    pub fn permission_state1(&self, options: &PushSubscriptionOptionsInit) -> Promise<PermissionState> {
        self.inner.call("permissionState", &[options.into(), ]).as_::<Promise<PermissionState>>()
    }
}
