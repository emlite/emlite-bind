use super::*;

/// The GetNotificationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetNotificationOptions {
    inner: Any,
}

impl FromVal for GetNotificationOptions {
    fn from_val(v: &Any) -> Self {
        GetNotificationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GetNotificationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GetNotificationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GetNotificationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GetNotificationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GetNotificationOptions> for Any {
    fn from(s: GetNotificationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GetNotificationOptions> for Any {
    fn from(s: &GetNotificationOptions) -> Any {
        s.inner.clone()
    }
}

impl GetNotificationOptions {
    /// Getter of the `tag` attribute.
    pub fn tag(&self) -> JsString {
        self.inner.get("tag").as_::<JsString>()
    }

    /// Setter of the `tag` attribute.
    pub fn set_tag(&mut self, value: &JsString) {
        self.inner.set("tag", value);
    }
}
