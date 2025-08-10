use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySystemMediaCapability {
    inner: Any,
}
impl FromVal for MediaKeySystemMediaCapability {
    fn from_val(v: &Any) -> Self {
        MediaKeySystemMediaCapability { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaKeySystemMediaCapability {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeySystemMediaCapability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaKeySystemMediaCapability {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaKeySystemMediaCapability {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaKeySystemMediaCapability> for Any {
    fn from(s: MediaKeySystemMediaCapability) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaKeySystemMediaCapability> for Any {
    fn from(s: &MediaKeySystemMediaCapability) -> Any {
        s.inner.clone()
    }
}

impl MediaKeySystemMediaCapability {
    pub fn content_type(&self) -> JsString {
        self.inner.get("contentType").as_::<JsString>()
    }

    pub fn set_content_type(&mut self, value: &JsString) {
        self.inner.set("contentType", value);
    }
}
impl MediaKeySystemMediaCapability {
    pub fn encryption_scheme(&self) -> JsString {
        self.inner.get("encryptionScheme").as_::<JsString>()
    }

    pub fn set_encryption_scheme(&mut self, value: &JsString) {
        self.inner.set("encryptionScheme", value);
    }
}
impl MediaKeySystemMediaCapability {
    pub fn robustness(&self) -> JsString {
        self.inner.get("robustness").as_::<JsString>()
    }

    pub fn set_robustness(&mut self, value: &JsString) {
        self.inner.set("robustness", value);
    }
}
