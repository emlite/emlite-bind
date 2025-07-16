use super::*;

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
/// The IDBObjectStore class.
/// [`IDBObjectStore`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBObjectStore {
    inner: Any,
}
impl FromVal for IDBObjectStore {
    fn from_val(v: &Any) -> Self {
        IDBObjectStore {
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
impl core::ops::Deref for IDBObjectStore {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBObjectStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IDBObjectStore {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IDBObjectStore {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IDBObjectStore> for Any {
    fn from(s: IDBObjectStore) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IDBObjectStore> for Any {
    fn from(s: &IDBObjectStore) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBObjectStore);

impl IDBObjectStore {
    /// Getter of the `name` attribute.
    /// [`IDBObjectStore.name`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    /// Setter of the `name` attribute.
    /// [`IDBObjectStore.name`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)
    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl IDBObjectStore {
    /// Getter of the `keyPath` attribute.
    /// [`IDBObjectStore.keyPath`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath)
    pub fn key_path(&self) -> Any {
        self.inner.get("keyPath").as_::<Any>()
    }
}
impl IDBObjectStore {
    /// Getter of the `indexNames` attribute.
    /// [`IDBObjectStore.indexNames`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)
    pub fn index_names(&self) -> DOMStringList {
        self.inner.get("indexNames").as_::<DOMStringList>()
    }
}
impl IDBObjectStore {
    /// Getter of the `transaction` attribute.
    /// [`IDBObjectStore.transaction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)
    pub fn transaction(&self) -> IDBTransaction {
        self.inner.get("transaction").as_::<IDBTransaction>()
    }
}
impl IDBObjectStore {
    /// Getter of the `autoIncrement` attribute.
    /// [`IDBObjectStore.autoIncrement`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)
    pub fn auto_increment(&self) -> bool {
        self.inner.get("autoIncrement").as_::<bool>()
    }
}
impl IDBObjectStore {
    /// The put method.
    /// [`IDBObjectStore.put`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)
    pub fn put0(&self, value: &Any) -> IDBRequest {
        self.inner.call("put", &[value.into()]).as_::<IDBRequest>()
    }
    /// The put method.
    /// [`IDBObjectStore.put`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)
    pub fn put1(&self, value: &Any, key: &Any) -> IDBRequest {
        self.inner
            .call("put", &[value.into(), key.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The add method.
    /// [`IDBObjectStore.add`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)
    pub fn add0(&self, value: &Any) -> IDBRequest {
        self.inner.call("add", &[value.into()]).as_::<IDBRequest>()
    }
    /// The add method.
    /// [`IDBObjectStore.add`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)
    pub fn add1(&self, value: &Any, key: &Any) -> IDBRequest {
        self.inner
            .call("add", &[value.into(), key.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The delete method.
    /// [`IDBObjectStore.delete`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)
    pub fn delete(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("delete", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The clear method.
    /// [`IDBObjectStore.clear`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)
    pub fn clear(&self) -> IDBRequest {
        self.inner.call("clear", &[]).as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The get method.
    /// [`IDBObjectStore.get`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)
    pub fn get(&self, query: &Any) -> IDBRequest {
        self.inner.call("get", &[query.into()]).as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The getKey method.
    /// [`IDBObjectStore.getKey`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)
    pub fn get_key(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("getKey", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The getAll method.
    /// [`IDBObjectStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)
    pub fn get_all0(&self) -> IDBRequest {
        self.inner.call("getAll", &[]).as_::<IDBRequest>()
    }
    /// The getAll method.
    /// [`IDBObjectStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)
    pub fn get_all1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("getAll", &[query.into()])
            .as_::<IDBRequest>()
    }
    /// The getAll method.
    /// [`IDBObjectStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)
    pub fn get_all2(&self, query: &Any, count: u32) -> IDBRequest {
        self.inner
            .call("getAll", &[query.into(), count.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The getAllKeys method.
    /// [`IDBObjectStore.getAllKeys`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    pub fn get_all_keys0(&self) -> IDBRequest {
        self.inner.call("getAllKeys", &[]).as_::<IDBRequest>()
    }
    /// The getAllKeys method.
    /// [`IDBObjectStore.getAllKeys`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    pub fn get_all_keys1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("getAllKeys", &[query.into()])
            .as_::<IDBRequest>()
    }
    /// The getAllKeys method.
    /// [`IDBObjectStore.getAllKeys`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)
    pub fn get_all_keys2(&self, query: &Any, count: u32) -> IDBRequest {
        self.inner
            .call("getAllKeys", &[query.into(), count.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The count method.
    /// [`IDBObjectStore.count`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)
    pub fn count0(&self) -> IDBRequest {
        self.inner.call("count", &[]).as_::<IDBRequest>()
    }
    /// The count method.
    /// [`IDBObjectStore.count`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)
    pub fn count1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("count", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The openCursor method.
    /// [`IDBObjectStore.openCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)
    pub fn open_cursor0(&self) -> IDBRequest {
        self.inner.call("openCursor", &[]).as_::<IDBRequest>()
    }
    /// The openCursor method.
    /// [`IDBObjectStore.openCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)
    pub fn open_cursor1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("openCursor", &[query.into()])
            .as_::<IDBRequest>()
    }
    /// The openCursor method.
    /// [`IDBObjectStore.openCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)
    pub fn open_cursor2(&self, query: &Any, direction: &IDBCursorDirection) -> IDBRequest {
        self.inner
            .call("openCursor", &[query.into(), direction.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The openKeyCursor method.
    /// [`IDBObjectStore.openKeyCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)
    pub fn open_key_cursor0(&self) -> IDBRequest {
        self.inner.call("openKeyCursor", &[]).as_::<IDBRequest>()
    }
    /// The openKeyCursor method.
    /// [`IDBObjectStore.openKeyCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)
    pub fn open_key_cursor1(&self, query: &Any) -> IDBRequest {
        self.inner
            .call("openKeyCursor", &[query.into()])
            .as_::<IDBRequest>()
    }
    /// The openKeyCursor method.
    /// [`IDBObjectStore.openKeyCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)
    pub fn open_key_cursor2(&self, query: &Any, direction: &IDBCursorDirection) -> IDBRequest {
        self.inner
            .call("openKeyCursor", &[query.into(), direction.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBObjectStore {
    /// The index method.
    /// [`IDBObjectStore.index`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)
    pub fn index(&self, name: &str) -> IDBIndex {
        self.inner.call("index", &[name.into()]).as_::<IDBIndex>()
    }
}
impl IDBObjectStore {
    /// The createIndex method.
    /// [`IDBObjectStore.createIndex`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
    pub fn create_index0(&self, name: &str, key_path: &Any) -> IDBIndex {
        self.inner
            .call("createIndex", &[name.into(), key_path.into()])
            .as_::<IDBIndex>()
    }
    /// The createIndex method.
    /// [`IDBObjectStore.createIndex`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)
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
    /// The deleteIndex method.
    /// [`IDBObjectStore.deleteIndex`](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)
    pub fn delete_index(&self, name: &str) -> Undefined {
        self.inner
            .call("deleteIndex", &[name.into()])
            .as_::<Undefined>()
    }
}
