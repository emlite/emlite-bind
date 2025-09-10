use super::*;

/// The NotificationEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationEventInit {
    inner: Any,
}

impl FromVal for NotificationEventInit {
    fn from_val(v: &Any) -> Self {
        NotificationEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NotificationEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NotificationEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NotificationEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NotificationEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NotificationEventInit> for Any {
    fn from(s: NotificationEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NotificationEventInit> for Any {
    fn from(s: &NotificationEventInit) -> Any {
        s.inner.clone()
    }
}

impl NotificationEventInit {
    /// Getter of the `notification` attribute.
    pub fn notification(&self) -> Notification {
        self.inner.get("notification").as_::<Notification>()
    }

    /// Setter of the `notification` attribute.
    pub fn set_notification(&mut self, value: &Notification) {
        self.inner.set("notification", value);
    }
}
impl NotificationEventInit {
    /// Getter of the `action` attribute.
    pub fn action(&self) -> JsString {
        self.inner.get("action").as_::<JsString>()
    }

    /// Setter of the `action` attribute.
    pub fn set_action(&mut self, value: &JsString) {
        self.inner.set("action", value);
    }
}
