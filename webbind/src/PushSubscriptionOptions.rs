use super::*;

/// The PushSubscriptionOptions class.
/// [`PushSubscriptionOptions`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionOptions {
    inner: Any,
}

impl FromVal for PushSubscriptionOptions {
    fn from_val(v: &Any) -> Self {
        PushSubscriptionOptions {
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

impl core::ops::Deref for PushSubscriptionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushSubscriptionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushSubscriptionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushSubscriptionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PushSubscriptionOptions> for Any {
    fn from(s: PushSubscriptionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushSubscriptionOptions> for Any {
    fn from(s: &PushSubscriptionOptions) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PushSubscriptionOptions);

impl PushSubscriptionOptions {
    /// Getter of the `userVisibleOnly` attribute.
    /// [`PushSubscriptionOptions.userVisibleOnly`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/userVisibleOnly)
    pub fn user_visible_only(&self) -> bool {
        self.inner.get("userVisibleOnly").as_::<bool>()
    }
}
impl PushSubscriptionOptions {
    /// Getter of the `applicationServerKey` attribute.
    /// [`PushSubscriptionOptions.applicationServerKey`](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/applicationServerKey)
    pub fn application_server_key(&self) -> ArrayBuffer {
        self.inner.get("applicationServerKey").as_::<ArrayBuffer>()
    }
}
