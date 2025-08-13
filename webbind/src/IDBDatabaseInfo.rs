use super::*;




/// The IDBDatabaseInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBDatabaseInfo {
    inner: Any,
}

impl FromVal for IDBDatabaseInfo {
    fn from_val(v: &Any) -> Self {
        IDBDatabaseInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBDatabaseInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBDatabaseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBDatabaseInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBDatabaseInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBDatabaseInfo> for Any {
    fn from(s: IDBDatabaseInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBDatabaseInfo> for Any {
    fn from(s: &IDBDatabaseInfo) -> Any {
        s.inner.clone()
    }
}

impl IDBDatabaseInfo {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl IDBDatabaseInfo {
    /// Getter of the `version` attribute.
    pub fn version(&self) -> u64 {
        self.inner.get("version").as_::<u64>()
    }

    /// Setter of the `version` attribute.
    pub fn set_version(&mut self, value: u64) {
        self.inner.set("version", value);
    }
}
