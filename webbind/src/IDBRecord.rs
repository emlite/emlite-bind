use super::*;




/// The IDBRecord class.
/// [`IDBRecord`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRecord)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBRecord {
    inner: Any,
}

impl FromVal for IDBRecord {
    fn from_val(v: &Any) -> Self {
        IDBRecord { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBRecord {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBRecord {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBRecord {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBRecord> for Any {
    fn from(s: IDBRecord) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBRecord> for Any {
    fn from(s: &IDBRecord) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBRecord);


impl IDBRecord {
    /// Getter of the `key` attribute.
    /// [`IDBRecord.key`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRecord/key)
    pub fn key(&self) -> Any {
        self.inner.get("key").as_::<Any>()
    }

}
impl IDBRecord {
    /// Getter of the `primaryKey` attribute.
    /// [`IDBRecord.primaryKey`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRecord/primaryKey)
    pub fn primary_key(&self) -> Any {
        self.inner.get("primaryKey").as_::<Any>()
    }

}
impl IDBRecord {
    /// Getter of the `value` attribute.
    /// [`IDBRecord.value`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRecord/value)
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

}
