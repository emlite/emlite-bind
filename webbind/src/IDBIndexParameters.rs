use super::*;




/// The IDBIndexParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBIndexParameters {
    inner: Any,
}

impl FromVal for IDBIndexParameters {
    fn from_val(v: &Any) -> Self {
        IDBIndexParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBIndexParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBIndexParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBIndexParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBIndexParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBIndexParameters> for Any {
    fn from(s: IDBIndexParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBIndexParameters> for Any {
    fn from(s: &IDBIndexParameters) -> Any {
        s.inner.clone()
    }
}

impl IDBIndexParameters {
    /// Getter of the `unique` attribute.
    pub fn unique(&self) -> bool {
        self.inner.get("unique").as_::<bool>()
    }

    /// Setter of the `unique` attribute.
    pub fn set_unique(&mut self, value: bool) {
        self.inner.set("unique", value);
    }
}
impl IDBIndexParameters {
    /// Getter of the `multiEntry` attribute.
    pub fn multi_entry(&self) -> bool {
        self.inner.get("multiEntry").as_::<bool>()
    }

    /// Setter of the `multiEntry` attribute.
    pub fn set_multi_entry(&mut self, value: bool) {
        self.inner.set("multiEntry", value);
    }
}
