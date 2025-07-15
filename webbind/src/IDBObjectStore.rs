use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBIndexParameters {
    inner: emlite::Val,
}
impl FromVal for IDBIndexParameters {
    fn from_val(v: &emlite::Val) -> Self {
        IDBIndexParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBIndexParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBIndexParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBIndexParameters {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBIndexParameters {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBIndexParameters> for emlite::Val {
    fn from(s: IDBIndexParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&IDBIndexParameters> for emlite::Val {
    fn from(s: &IDBIndexParameters) -> emlite::Val {
        s.inner.clone()
    }
}

impl IDBIndexParameters {
    pub fn unique(&self) -> bool {
        self.inner.get("unique").as_::<bool>()
    }

    pub fn set_unique(&mut self, value: bool) {
        self.inner.set("unique", value);
    }
}
impl IDBIndexParameters {
    pub fn multi_entry(&self) -> bool {
        self.inner.get("multiEntry").as_::<bool>()
    }

    pub fn set_multi_entry(&mut self, value: bool) {
        self.inner.set("multiEntry", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBObjectStore {
    inner: emlite::Val,
}
impl FromVal for IDBObjectStore {
    fn from_val(v: &emlite::Val) -> Self {
        IDBObjectStore {
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
impl core::ops::Deref for IDBObjectStore {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBObjectStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBObjectStore {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBObjectStore {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBObjectStore> for emlite::Val {
    fn from(s: IDBObjectStore) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&IDBObjectStore> for emlite::Val {
    fn from(s: &IDBObjectStore) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBObjectStore);

impl IDBObjectStore {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl IDBObjectStore {
    pub fn key_path(&self) -> Any {
        self.inner.get("keyPath").as_::<Any>()
    }
}
impl IDBObjectStore {
    pub fn index_names(&self) -> DOMStringList {
        self.inner.get("indexNames").as_::<DOMStringList>()
    }
}
impl IDBObjectStore {
    pub fn transaction(&self) -> IDBTransaction {
        self.inner.get("transaction").as_::<IDBTransaction>()
    }
}
impl IDBObjectStore {
    pub fn auto_increment(&self) -> bool {
        self.inner.get("autoIncrement").as_::<bool>()
    }
}
impl IDBObjectStore {
    pub fn put0(&self, value: &Any) -> IDBRequest {
        self.inner.call("put", &[value.into()]).as_::<IDBRequest>()
    }

    pub fn put1(&self, value: &Any, key: &Any) -> IDBRequest {
        self.inner
            .call("put", &[value.into(), key.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn add0(&self, value: &Any) -> IDBRequest {
        self.inner.call("add", &[value.into()]).as_::<IDBRequest>()
    }

    pub fn add1(&self, value: &Any, key: &Any) -> IDBRequest {
        self.inner
            .call("add", &[value.into(), key.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn delete(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("delete", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn clear(&self) -> IDBRequest {
        self.inner.call("clear", &[]).as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn get(&self, query: &Any) -> IDBRequest {
        self.inner.call("get", &[query.into()]).as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn get_key(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("getKey", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn get_all0(&self) -> IDBRequest {
        self.inner.call("getAll", &[]).as_::<IDBRequest>()
    }

    pub fn get_all1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("getAll", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn get_all2(&self, query: &Any, count: u32) -> IDBRequest {
        self.inner
            .call("getAll", &[query.into(), count.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn get_all_keys0(&self) -> IDBRequest {
        self.inner.call("getAllKeys", &[]).as_::<IDBRequest>()
    }

    pub fn get_all_keys1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("getAllKeys", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn get_all_keys2(&self, query: &Any, count: u32) -> IDBRequest {
        self.inner
            .call("getAllKeys", &[query.into(), count.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn count0(&self) -> IDBRequest {
        self.inner.call("count", &[]).as_::<IDBRequest>()
    }

    pub fn count1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("count", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn open_cursor0(&self) -> IDBRequest {
        self.inner.call("openCursor", &[]).as_::<IDBRequest>()
    }

    pub fn open_cursor1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("openCursor", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn open_cursor2(&self, query: &Any, direction: &IDBCursorDirection) -> IDBRequest {
        self.inner
            .call("openCursor", &[query.into(), direction.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn open_key_cursor0(&self) -> IDBRequest {
        self.inner.call("openKeyCursor", &[]).as_::<IDBRequest>()
    }

    pub fn open_key_cursor1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("openKeyCursor", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn open_key_cursor2(&self, query: &Any, direction: &IDBCursorDirection) -> IDBRequest {
        self.inner
            .call("openKeyCursor", &[query.into(), direction.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    pub fn index(&self, name: &str) -> IDBIndex {
        self.inner.call("index", &[name.into()]).as_::<IDBIndex>()
    }
}
impl IDBObjectStore {
    pub fn create_index0(&self, name: &str, key_path: &Any) -> IDBIndex {
        self.inner
            .call("createIndex", &[name.into(), key_path.into()])
            .as_::<IDBIndex>()
    }

    pub fn create_index1(
        &self,
        name: &str,
        key_path: &Any,
        options: &IDBIndexParameters,
    ) -> IDBIndex {
        self.inner
            .call(
                "createIndex",
                &[name.into(), key_path.into(), options.into()],
            )
            .as_::<IDBIndex>()
    }
}
impl IDBObjectStore {
    pub fn delete_index(&self, name: &str) -> Undefined {
        self.inner
            .call("deleteIndex", &[name.into()])
            .as_::<Undefined>()
    }
}
