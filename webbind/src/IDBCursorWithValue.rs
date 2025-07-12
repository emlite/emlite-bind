use super::*;

#[derive(Clone, Debug)]
pub struct IDBCursorWithValue {
    inner: IDBCursor,
}
impl FromVal for IDBCursorWithValue {
    fn from_val(v: &emlite::Val) -> Self {
        IDBCursorWithValue {
            inner: IDBCursor::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBCursorWithValue {
    type Target = IDBCursor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBCursorWithValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBCursorWithValue> for emlite::Val {
    fn from(s: IDBCursorWithValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBCursorWithValue {
    pub fn value(&self) -> jsbind::Any {
        self.inner.get("value").as_::<jsbind::Any>()
    }
}
