use super::*;




/// The PushSubscriptionOptionsInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionOptionsInit {
    inner: Any,
}

impl FromVal for PushSubscriptionOptionsInit {
    fn from_val(v: &Any) -> Self {
        PushSubscriptionOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PushSubscriptionOptionsInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushSubscriptionOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushSubscriptionOptionsInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushSubscriptionOptionsInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PushSubscriptionOptionsInit> for Any {
    fn from(s: PushSubscriptionOptionsInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushSubscriptionOptionsInit> for Any {
    fn from(s: &PushSubscriptionOptionsInit) -> Any {
        s.inner.clone()
    }
}

impl PushSubscriptionOptionsInit {
    /// Getter of the `userVisibleOnly` attribute.
    pub fn user_visible_only(&self) -> bool {
        self.inner.get("userVisibleOnly").as_::<bool>()
    }

    /// Setter of the `userVisibleOnly` attribute.
    pub fn set_user_visible_only(&mut self, value: bool) {
        self.inner.set("userVisibleOnly", value);
    }
}
impl PushSubscriptionOptionsInit {
    /// Getter of the `applicationServerKey` attribute.
    pub fn application_server_key(&self) -> Any {
        self.inner.get("applicationServerKey").as_::<Any>()
    }

    /// Setter of the `applicationServerKey` attribute.
    pub fn set_application_server_key(&mut self, value: &Any) {
        self.inner.set("applicationServerKey", value);
    }
}
