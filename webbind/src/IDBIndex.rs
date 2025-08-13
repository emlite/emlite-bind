use super::*;




/// The IDBIndex class.
/// [`IDBIndex`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBIndex {
    inner: Any,
}

impl FromVal for IDBIndex {
    fn from_val(v: &Any) -> Self {
        IDBIndex { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBIndex {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBIndex {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBIndex {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBIndex> for Any {
    fn from(s: IDBIndex) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBIndex> for Any {
    fn from(s: &IDBIndex) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBIndex);


impl IDBIndex {
    /// Getter of the `name` attribute.
    /// [`IDBIndex.name`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`IDBIndex.name`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl IDBIndex {
    /// Getter of the `objectStore` attribute.
    /// [`IDBIndex.objectStore`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/objectStore)
    pub fn object_store(&self) -> IDBObjectStore {
        self.inner.get("objectStore").as_::<IDBObjectStore>()
    }

}
impl IDBIndex {
    /// Getter of the `keyPath` attribute.
    /// [`IDBIndex.keyPath`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/keyPath)
    pub fn key_path(&self) -> Any {
        self.inner.get("keyPath").as_::<Any>()
    }

}
impl IDBIndex {
    /// Getter of the `multiEntry` attribute.
    /// [`IDBIndex.multiEntry`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry)
    pub fn multi_entry(&self) -> bool {
        self.inner.get("multiEntry").as_::<bool>()
    }

}
impl IDBIndex {
    /// Getter of the `unique` attribute.
    /// [`IDBIndex.unique`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique)
    pub fn unique(&self) -> bool {
        self.inner.get("unique").as_::<bool>()
    }

}
impl IDBIndex {
    /// The get method.
    /// [`IDBIndex.get`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/get)
    pub fn get(&self, query: &Any) -> IDBRequest {
        self.inner.call("get", &[query.into(), ]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    /// The getKey method.
    /// [`IDBIndex.getKey`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getKey)
    pub fn get_key(&self, query: &Any) -> IDBRequest {
        self.inner.call("getKey", &[query.into(), ]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    /// The getAll method.
    /// [`IDBIndex.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    pub fn get_all0(&self, ) -> IDBRequest {
        self.inner.call("getAll", &[]).as_::<IDBRequest>()
    }
    /// The getAll method.
    /// [`IDBIndex.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    pub fn get_all1(&self, query: &Any) -> IDBRequest {
        self.inner.call("getAll", &[query.into(), ]).as_::<IDBRequest>()
    }
    /// The getAll method.
    /// [`IDBIndex.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)
    pub fn get_all2(&self, query: &Any, count: u32) -> IDBRequest {
        self.inner.call("getAll", &[query.into(), count.into(), ]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    /// The getAllKeys method.
    /// [`IDBIndex.getAllKeys`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)
    pub fn get_all_keys0(&self, ) -> IDBRequest {
        self.inner.call("getAllKeys", &[]).as_::<IDBRequest>()
    }
    /// The getAllKeys method.
    /// [`IDBIndex.getAllKeys`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)
    pub fn get_all_keys1(&self, query: &Any) -> IDBRequest {
        self.inner.call("getAllKeys", &[query.into(), ]).as_::<IDBRequest>()
    }
    /// The getAllKeys method.
    /// [`IDBIndex.getAllKeys`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)
    pub fn get_all_keys2(&self, query: &Any, count: u32) -> IDBRequest {
        self.inner.call("getAllKeys", &[query.into(), count.into(), ]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    /// The count method.
    /// [`IDBIndex.count`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)
    pub fn count0(&self, ) -> IDBRequest {
        self.inner.call("count", &[]).as_::<IDBRequest>()
    }
    /// The count method.
    /// [`IDBIndex.count`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)
    pub fn count1(&self, query: &Any) -> IDBRequest {
        self.inner.call("count", &[query.into(), ]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    /// The openCursor method.
    /// [`IDBIndex.openCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    pub fn open_cursor0(&self, ) -> IDBRequest {
        self.inner.call("openCursor", &[]).as_::<IDBRequest>()
    }
    /// The openCursor method.
    /// [`IDBIndex.openCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    pub fn open_cursor1(&self, query: &Any) -> IDBRequest {
        self.inner.call("openCursor", &[query.into(), ]).as_::<IDBRequest>()
    }
    /// The openCursor method.
    /// [`IDBIndex.openCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)
    pub fn open_cursor2(&self, query: &Any, direction: &IDBCursorDirection) -> IDBRequest {
        self.inner.call("openCursor", &[query.into(), direction.into(), ]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    /// The openKeyCursor method.
    /// [`IDBIndex.openKeyCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    pub fn open_key_cursor0(&self, ) -> IDBRequest {
        self.inner.call("openKeyCursor", &[]).as_::<IDBRequest>()
    }
    /// The openKeyCursor method.
    /// [`IDBIndex.openKeyCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    pub fn open_key_cursor1(&self, query: &Any) -> IDBRequest {
        self.inner.call("openKeyCursor", &[query.into(), ]).as_::<IDBRequest>()
    }
    /// The openKeyCursor method.
    /// [`IDBIndex.openKeyCursor`](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)
    pub fn open_key_cursor2(&self, query: &Any, direction: &IDBCursorDirection) -> IDBRequest {
        self.inner.call("openKeyCursor", &[query.into(), direction.into(), ]).as_::<IDBRequest>()
    }
}
