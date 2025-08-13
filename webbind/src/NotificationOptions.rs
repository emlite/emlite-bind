use super::*;




/// The NotificationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NotificationOptions {
    inner: Any,
}

impl FromVal for NotificationOptions {
    fn from_val(v: &Any) -> Self {
        NotificationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NotificationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NotificationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NotificationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NotificationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NotificationOptions> for Any {
    fn from(s: NotificationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NotificationOptions> for Any {
    fn from(s: &NotificationOptions) -> Any {
        s.inner.clone()
    }
}

impl NotificationOptions {
    /// Getter of the `dir` attribute.
    pub fn dir(&self) -> NotificationDirection {
        self.inner.get("dir").as_::<NotificationDirection>()
    }

    /// Setter of the `dir` attribute.
    pub fn set_dir(&mut self, value: &NotificationDirection) {
        self.inner.set("dir", value);
    }
}
impl NotificationOptions {
    /// Getter of the `lang` attribute.
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

    /// Setter of the `lang` attribute.
    pub fn set_lang(&mut self, value: &JsString) {
        self.inner.set("lang", value);
    }
}
impl NotificationOptions {
    /// Getter of the `body` attribute.
    pub fn body(&self) -> JsString {
        self.inner.get("body").as_::<JsString>()
    }

    /// Setter of the `body` attribute.
    pub fn set_body(&mut self, value: &JsString) {
        self.inner.set("body", value);
    }
}
impl NotificationOptions {
    /// Getter of the `tag` attribute.
    pub fn tag(&self) -> JsString {
        self.inner.get("tag").as_::<JsString>()
    }

    /// Setter of the `tag` attribute.
    pub fn set_tag(&mut self, value: &JsString) {
        self.inner.set("tag", value);
    }
}
impl NotificationOptions {
    /// Getter of the `image` attribute.
    pub fn image(&self) -> JsString {
        self.inner.get("image").as_::<JsString>()
    }

    /// Setter of the `image` attribute.
    pub fn set_image(&mut self, value: &JsString) {
        self.inner.set("image", value);
    }
}
impl NotificationOptions {
    /// Getter of the `icon` attribute.
    pub fn icon(&self) -> JsString {
        self.inner.get("icon").as_::<JsString>()
    }

    /// Setter of the `icon` attribute.
    pub fn set_icon(&mut self, value: &JsString) {
        self.inner.set("icon", value);
    }
}
impl NotificationOptions {
    /// Getter of the `badge` attribute.
    pub fn badge(&self) -> JsString {
        self.inner.get("badge").as_::<JsString>()
    }

    /// Setter of the `badge` attribute.
    pub fn set_badge(&mut self, value: &JsString) {
        self.inner.set("badge", value);
    }
}
impl NotificationOptions {
    /// Getter of the `vibrate` attribute.
    pub fn vibrate(&self) -> Any {
        self.inner.get("vibrate").as_::<Any>()
    }

    /// Setter of the `vibrate` attribute.
    pub fn set_vibrate(&mut self, value: &Any) {
        self.inner.set("vibrate", value);
    }
}
impl NotificationOptions {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: &Any) {
        self.inner.set("timestamp", value);
    }
}
impl NotificationOptions {
    /// Getter of the `renotify` attribute.
    pub fn renotify(&self) -> bool {
        self.inner.get("renotify").as_::<bool>()
    }

    /// Setter of the `renotify` attribute.
    pub fn set_renotify(&mut self, value: bool) {
        self.inner.set("renotify", value);
    }
}
impl NotificationOptions {
    /// Getter of the `silent` attribute.
    pub fn silent(&self) -> bool {
        self.inner.get("silent").as_::<bool>()
    }

    /// Setter of the `silent` attribute.
    pub fn set_silent(&mut self, value: bool) {
        self.inner.set("silent", value);
    }
}
impl NotificationOptions {
    /// Getter of the `requireInteraction` attribute.
    pub fn require_interaction(&self) -> bool {
        self.inner.get("requireInteraction").as_::<bool>()
    }

    /// Setter of the `requireInteraction` attribute.
    pub fn set_require_interaction(&mut self, value: bool) {
        self.inner.set("requireInteraction", value);
    }
}
impl NotificationOptions {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl NotificationOptions {
    /// Getter of the `actions` attribute.
    pub fn actions(&self) -> TypedArray<NotificationAction> {
        self.inner.get("actions").as_::<TypedArray<NotificationAction>>()
    }

    /// Setter of the `actions` attribute.
    pub fn set_actions(&mut self, value: &TypedArray<NotificationAction>) {
        self.inner.set("actions", value);
    }
}
