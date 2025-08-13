use super::*;




/// The IDBObjectStoreParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBObjectStoreParameters {
    inner: Any,
}

impl FromVal for IDBObjectStoreParameters {
    fn from_val(v: &Any) -> Self {
        IDBObjectStoreParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBObjectStoreParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBObjectStoreParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBObjectStoreParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBObjectStoreParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBObjectStoreParameters> for Any {
    fn from(s: IDBObjectStoreParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBObjectStoreParameters> for Any {
    fn from(s: &IDBObjectStoreParameters) -> Any {
        s.inner.clone()
    }
}

impl IDBObjectStoreParameters {
    /// Getter of the `keyPath` attribute.
    pub fn key_path(&self) -> Any {
        self.inner.get("keyPath").as_::<Any>()
    }

    /// Setter of the `keyPath` attribute.
    pub fn set_key_path(&mut self, value: &Any) {
        self.inner.set("keyPath", value);
    }
}
impl IDBObjectStoreParameters {
    /// Getter of the `autoIncrement` attribute.
    pub fn auto_increment(&self) -> bool {
        self.inner.get("autoIncrement").as_::<bool>()
    }

    /// Setter of the `autoIncrement` attribute.
    pub fn set_auto_increment(&mut self, value: bool) {
        self.inner.set("autoIncrement", value);
    }
}
