use super::*;

/// The NDEFRecordInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFRecordInit {
    inner: Any,
}

impl FromVal for NDEFRecordInit {
    fn from_val(v: &Any) -> Self {
        NDEFRecordInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NDEFRecordInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NDEFRecordInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NDEFRecordInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NDEFRecordInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NDEFRecordInit> for Any {
    fn from(s: NDEFRecordInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NDEFRecordInit> for Any {
    fn from(s: &NDEFRecordInit) -> Any {
        s.inner.clone()
    }
}

impl NDEFRecordInit {
    /// Getter of the `recordType` attribute.
    pub fn record_type(&self) -> JsString {
        self.inner.get("recordType").as_::<JsString>()
    }

    /// Setter of the `recordType` attribute.
    pub fn set_record_type(&mut self, value: &JsString) {
        self.inner.set("recordType", value);
    }
}
impl NDEFRecordInit {
    /// Getter of the `mediaType` attribute.
    pub fn media_type(&self) -> JsString {
        self.inner.get("mediaType").as_::<JsString>()
    }

    /// Setter of the `mediaType` attribute.
    pub fn set_media_type(&mut self, value: &JsString) {
        self.inner.set("mediaType", value);
    }
}
impl NDEFRecordInit {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl NDEFRecordInit {
    /// Getter of the `encoding` attribute.
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }

    /// Setter of the `encoding` attribute.
    pub fn set_encoding(&mut self, value: &JsString) {
        self.inner.set("encoding", value);
    }
}
impl NDEFRecordInit {
    /// Getter of the `lang` attribute.
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

    /// Setter of the `lang` attribute.
    pub fn set_lang(&mut self, value: &JsString) {
        self.inner.set("lang", value);
    }
}
impl NDEFRecordInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
