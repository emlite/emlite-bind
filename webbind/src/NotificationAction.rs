use super::*;




/// The NotificationAction dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationAction {
    inner: Any,
}

impl FromVal for NotificationAction {
    fn from_val(v: &Any) -> Self {
        NotificationAction { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NotificationAction {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NotificationAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NotificationAction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NotificationAction {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NotificationAction> for Any {
    fn from(s: NotificationAction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NotificationAction> for Any {
    fn from(s: &NotificationAction) -> Any {
        s.inner.clone()
    }
}

impl NotificationAction {
    /// Getter of the `action` attribute.
    pub fn action(&self) -> JsString {
        self.inner.get("action").as_::<JsString>()
    }

    /// Setter of the `action` attribute.
    pub fn set_action(&mut self, value: &JsString) {
        self.inner.set("action", value);
    }
}
impl NotificationAction {
    /// Getter of the `title` attribute.
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    /// Setter of the `title` attribute.
    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl NotificationAction {
    /// Getter of the `navigate` attribute.
    pub fn navigate(&self) -> JsString {
        self.inner.get("navigate").as_::<JsString>()
    }

    /// Setter of the `navigate` attribute.
    pub fn set_navigate(&mut self, value: &JsString) {
        self.inner.set("navigate", value);
    }
}
impl NotificationAction {
    /// Getter of the `icon` attribute.
    pub fn icon(&self) -> JsString {
        self.inner.get("icon").as_::<JsString>()
    }

    /// Setter of the `icon` attribute.
    pub fn set_icon(&mut self, value: &JsString) {
        self.inner.set("icon", value);
    }
}
