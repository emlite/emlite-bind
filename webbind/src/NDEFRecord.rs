use super::*;




/// The NDEFRecord class.
/// [`NDEFRecord`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFRecord {
    inner: Any,
}

impl FromVal for NDEFRecord {
    fn from_val(v: &Any) -> Self {
        NDEFRecord { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NDEFRecord {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NDEFRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NDEFRecord {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NDEFRecord {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NDEFRecord> for Any {
    fn from(s: NDEFRecord) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NDEFRecord> for Any {
    fn from(s: &NDEFRecord) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NDEFRecord);



impl NDEFRecord {
    /// The `new NDEFRecord(..)` constructor, creating a new NDEFRecord instance
    pub fn new(record_init: &NDEFRecordInit) -> NDEFRecord {
        Self {
            inner: Any::global("NDEFRecord").new(&[record_init.into()]).as_::<Any>(),
        }
    }

}
impl NDEFRecord {
    /// Getter of the `recordType` attribute.
    /// [`NDEFRecord.recordType`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/recordType)
    pub fn record_type(&self) -> JsString {
        self.inner.get("recordType").as_::<JsString>()
    }

}
impl NDEFRecord {
    /// Getter of the `mediaType` attribute.
    /// [`NDEFRecord.mediaType`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/mediaType)
    pub fn media_type(&self) -> JsString {
        self.inner.get("mediaType").as_::<JsString>()
    }

}
impl NDEFRecord {
    /// Getter of the `id` attribute.
    /// [`NDEFRecord.id`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

}
impl NDEFRecord {
    /// Getter of the `data` attribute.
    /// [`NDEFRecord.data`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/data)
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }

}
impl NDEFRecord {
    /// Getter of the `encoding` attribute.
    /// [`NDEFRecord.encoding`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/encoding)
    pub fn encoding(&self) -> JsString {
        self.inner.get("encoding").as_::<JsString>()
    }

}
impl NDEFRecord {
    /// Getter of the `lang` attribute.
    /// [`NDEFRecord.lang`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/lang)
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

}
impl NDEFRecord {
    /// The toRecords method.
    /// [`NDEFRecord.toRecords`](https://developer.mozilla.org/en-US/docs/Web/API/NDEFRecord/toRecords)
    pub fn to_records(&self, ) -> TypedArray<NDEFRecord> {
        self.inner.call("toRecords", &[]).as_::<TypedArray<NDEFRecord>>()
    }
}
