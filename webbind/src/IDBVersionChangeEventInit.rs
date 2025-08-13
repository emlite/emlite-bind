use super::*;




/// The IDBVersionChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBVersionChangeEventInit {
    inner: Any,
}

impl FromVal for IDBVersionChangeEventInit {
    fn from_val(v: &Any) -> Self {
        IDBVersionChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBVersionChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBVersionChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBVersionChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBVersionChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBVersionChangeEventInit> for Any {
    fn from(s: IDBVersionChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBVersionChangeEventInit> for Any {
    fn from(s: &IDBVersionChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl IDBVersionChangeEventInit {
    /// Getter of the `oldVersion` attribute.
    pub fn old_version(&self) -> u64 {
        self.inner.get("oldVersion").as_::<u64>()
    }

    /// Setter of the `oldVersion` attribute.
    pub fn set_old_version(&mut self, value: u64) {
        self.inner.set("oldVersion", value);
    }
}
impl IDBVersionChangeEventInit {
    /// Getter of the `newVersion` attribute.
    pub fn new_version(&self) -> u64 {
        self.inner.get("newVersion").as_::<u64>()
    }

    /// Setter of the `newVersion` attribute.
    pub fn set_new_version(&mut self, value: u64) {
        self.inner.set("newVersion", value);
    }
}
