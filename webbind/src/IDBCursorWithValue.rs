use super::*;

/// The IDBCursorWithValue class.
/// [`IDBCursorWithValue`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBCursorWithValue {
    inner: IDBCursor,
}
impl FromVal for IDBCursorWithValue {
    fn from_val(v: &Any) -> Self {
        IDBCursorWithValue {
            inner: IDBCursor::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBCursorWithValue {
    type Target = IDBCursor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBCursorWithValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IDBCursorWithValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IDBCursorWithValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IDBCursorWithValue> for Any {
    fn from(s: IDBCursorWithValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IDBCursorWithValue> for Any {
    fn from(s: &IDBCursorWithValue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBCursorWithValue);

impl IDBCursorWithValue {
    /// Getter of the `value` attribute.
    /// [`IDBCursorWithValue.value`](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }
}
