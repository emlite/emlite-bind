use super::*;

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
    pub fn record_type(&self) -> JsString {
        self.inner.get("recordType").as_::<JsString>()
    }

    pub fn set_record_type(&mut self, value: &JsString) {
        self.inner.set("recordType", value);
    }
}
impl NDEFRecordInit {
    pub fn media_type(&self) -> JsString {
        self.inner.get("mediaType").as_::<JsString>()
    }

    pub fn set_media_type(&mut self, value: &JsString) {
        self.inner.set("mediaType", value);
    }
}
impl NDEFRecordInit {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl NDEFRecordInit {
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }

    pub fn set_encoding(&mut self, value: &JsString) {
        self.inner.set("encoding", value);
    }
}
impl NDEFRecordInit {
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

    pub fn set_lang(&mut self, value: &JsString) {
        self.inner.set("lang", value);
    }
}
impl NDEFRecordInit {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
