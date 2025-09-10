use super::*;

/// The IDBGetAllOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBGetAllOptions {
    inner: Any,
}

impl FromVal for IDBGetAllOptions {
    fn from_val(v: &Any) -> Self {
        IDBGetAllOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBGetAllOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBGetAllOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBGetAllOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBGetAllOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBGetAllOptions> for Any {
    fn from(s: IDBGetAllOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBGetAllOptions> for Any {
    fn from(s: &IDBGetAllOptions) -> Any {
        s.inner.clone()
    }
}

impl IDBGetAllOptions {
    /// Getter of the `query` attribute.
    pub fn query(&self) -> Any {
        self.inner.get("query").as_::<Any>()
    }

    /// Setter of the `query` attribute.
    pub fn set_query(&mut self, value: &Any) {
        self.inner.set("query", value);
    }
}
impl IDBGetAllOptions {
    /// Getter of the `count` attribute.
    pub fn count(&self) -> u32 {
        self.inner.get("count").as_::<u32>()
    }

    /// Setter of the `count` attribute.
    pub fn set_count(&mut self, value: u32) {
        self.inner.set("count", value);
    }
}
impl IDBGetAllOptions {
    /// Getter of the `direction` attribute.
    pub fn direction(&self) -> IDBCursorDirection {
        self.inner.get("direction").as_::<IDBCursorDirection>()
    }

    /// Setter of the `direction` attribute.
    pub fn set_direction(&mut self, value: &IDBCursorDirection) {
        self.inner.set("direction", value);
    }
}
