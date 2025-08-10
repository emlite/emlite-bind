use super::*;

/// The PushSubscriptionChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionChangeEventInit {
    inner: Any,
}

impl FromVal for PushSubscriptionChangeEventInit {
    fn from_val(v: &Any) -> Self {
        PushSubscriptionChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PushSubscriptionChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PushSubscriptionChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PushSubscriptionChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PushSubscriptionChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PushSubscriptionChangeEventInit> for Any {
    fn from(s: PushSubscriptionChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PushSubscriptionChangeEventInit> for Any {
    fn from(s: &PushSubscriptionChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl PushSubscriptionChangeEventInit {
    /// Getter of the `newSubscription` attribute.
    pub fn new_subscription(&self) -> PushSubscription {
        self.inner.get("newSubscription").as_::<PushSubscription>()
    }

    /// Setter of the `newSubscription` attribute.
    pub fn set_new_subscription(&mut self, value: &PushSubscription) {
        self.inner.set("newSubscription", value);
    }
}
impl PushSubscriptionChangeEventInit {
    /// Getter of the `oldSubscription` attribute.
    pub fn old_subscription(&self) -> PushSubscription {
        self.inner.get("oldSubscription").as_::<PushSubscription>()
    }

    /// Setter of the `oldSubscription` attribute.
    pub fn set_old_subscription(&mut self, value: &PushSubscription) {
        self.inner.set("oldSubscription", value);
    }
}
