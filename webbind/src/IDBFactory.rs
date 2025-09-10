use super::*;

/// The IDBFactory class.
/// [`IDBFactory`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBFactory {
    inner: Any,
}

impl FromVal for IDBFactory {
    fn from_val(v: &Any) -> Self {
        IDBFactory {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBFactory {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBFactory {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBFactory {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBFactory> for Any {
    fn from(s: IDBFactory) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBFactory> for Any {
    fn from(s: &IDBFactory) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBFactory);

impl IDBFactory {
    /// The open method.
    /// [`IDBFactory.open`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)
    pub fn open0(&self, name: &JsString) -> IDBOpenDBRequest {
        self.inner
            .call("open", &[name.into()])
            .as_::<IDBOpenDBRequest>()
    }
    /// The open method.
    /// [`IDBFactory.open`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)
    pub fn open1(&self, name: &JsString, version: u64) -> IDBOpenDBRequest {
        self.inner
            .call("open", &[name.into(), version.into()])
            .as_::<IDBOpenDBRequest>()
    }
}
impl IDBFactory {
    /// The deleteDatabase method.
    /// [`IDBFactory.deleteDatabase`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)
    pub fn delete_database(&self, name: &JsString) -> IDBOpenDBRequest {
        self.inner
            .call("deleteDatabase", &[name.into()])
            .as_::<IDBOpenDBRequest>()
    }
}
impl IDBFactory {
    /// The databases method.
    /// [`IDBFactory.databases`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/databases)
    pub fn databases(&self) -> Promise<TypedArray<IDBDatabaseInfo>> {
        self.inner
            .call("databases", &[])
            .as_::<Promise<TypedArray<IDBDatabaseInfo>>>()
    }
}
impl IDBFactory {
    /// The cmp method.
    /// [`IDBFactory.cmp`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp)
    pub fn cmp(&self, first: &Any, second: &Any) -> i16 {
        self.inner
            .call("cmp", &[first.into(), second.into()])
            .as_::<i16>()
    }
}
