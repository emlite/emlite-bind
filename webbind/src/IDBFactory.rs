use super::*;

#[derive(Clone, Debug)]
pub struct IDBDatabaseInfo {
    inner: emlite::Val,
}
impl FromVal for IDBDatabaseInfo {
    fn from_val(v: &emlite::Val) -> Self {
        IDBDatabaseInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBDatabaseInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBDatabaseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBDatabaseInfo> for emlite::Val {
    fn from(s: IDBDatabaseInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBDatabaseInfo {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
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
#[derive(Clone, Debug)]
pub struct IDBFactory {
    inner: emlite::Val,
}
impl FromVal for IDBFactory {
    fn from_val(v: &emlite::Val) -> Self {
        IDBFactory {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for IDBFactory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IDBFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBFactory> for emlite::Val {
    fn from(s: IDBFactory) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBFactory {
    pub fn open0(&self, name: jsbind::DOMString) -> IDBOpenDBRequest {
        self.inner
            .call("open", &[name.into()])
            .as_::<IDBOpenDBRequest>()
    }

    pub fn open1(&self, name: jsbind::DOMString, version: u64) -> IDBOpenDBRequest {
        self.inner
            .call("open", &[name.into(), version.into()])
            .as_::<IDBOpenDBRequest>()
    }
}
impl IDBFactory {
    pub fn delete_database(&self, name: jsbind::DOMString) -> IDBOpenDBRequest {
        self.inner
            .call("deleteDatabase", &[name.into()])
            .as_::<IDBOpenDBRequest>()
    }
}
impl IDBFactory {
    pub fn databases(&self) -> jsbind::Promise {
        self.inner.call("databases", &[]).as_::<jsbind::Promise>()
    }
}
impl IDBFactory {
    pub fn cmp(&self, first: jsbind::Any, second: jsbind::Any) -> i16 {
        self.inner
            .call("cmp", &[first.into(), second.into()])
            .as_::<i16>()
    }
}
