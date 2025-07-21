use super::*;

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
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl IDBDatabaseInfo {
    pub fn version(&self) -> u64 {
        self.inner.get("version").as_::<u64>()
    }

    pub fn set_version(&mut self, value: u64) {
        self.inner.set("version", value);
    }
}
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
    pub fn open0(&self, name: &str) -> IDBOpenDBRequest {
        self.inner
            .call("open", &[name.into()])
            .as_::<IDBOpenDBRequest>()
    }
    /// The open method.
    /// [`IDBFactory.open`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)
    pub fn open1(&self, name: &str, version: u64) -> IDBOpenDBRequest {
        self.inner
            .call("open", &[name.into(), version.into()])
            .as_::<IDBOpenDBRequest>()
    }
}
impl IDBFactory {
    /// The deleteDatabase method.
    /// [`IDBFactory.deleteDatabase`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)
    pub fn delete_database(&self, name: &str) -> IDBOpenDBRequest {
        self.inner
            .call("deleteDatabase", &[name.into()])
            .as_::<IDBOpenDBRequest>()
    }
}
impl IDBFactory {
    /// The databases method.
    /// [`IDBFactory.databases`](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/databases)
    pub fn databases(&self) -> Promise<Sequence<IDBDatabaseInfo>> {
        self.inner
            .call("databases", &[])
            .as_::<Promise<Sequence<IDBDatabaseInfo>>>()
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
