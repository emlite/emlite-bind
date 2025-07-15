use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBIndex {
    inner: emlite::Val,
}
impl FromVal for IDBIndex {
    fn from_val(v: &emlite::Val) -> Self {
        IDBIndex {
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
impl core::ops::Deref for IDBIndex {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBIndex {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBIndex {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBIndex> for emlite::Val {
    fn from(s: IDBIndex) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IDBIndex);

impl IDBIndex {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl IDBIndex {
    pub fn object_store(&self) -> IDBObjectStore {
        self.inner.get("objectStore").as_::<IDBObjectStore>()
    }
}
impl IDBIndex {
    pub fn key_path(&self) -> Any {
        self.inner.get("keyPath").as_::<Any>()
    }
}
impl IDBIndex {
    pub fn multi_entry(&self) -> bool {
        self.inner.get("multiEntry").as_::<bool>()
    }
}
impl IDBIndex {
    pub fn unique(&self) -> bool {
        self.inner.get("unique").as_::<bool>()
    }
}
impl IDBIndex {
    pub fn get(&self, query: Any) -> IDBRequest {
        self.inner.call("get", &[query.into()]).as_::<IDBRequest>()
    }
}
impl IDBIndex {
    pub fn get_key(&self, query: Any) -> IDBRequest {
        self.inner
            .call("getKey", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBIndex {
    pub fn get_all0(&self) -> IDBRequest {
        self.inner.call("getAll", &[]).as_::<IDBRequest>()
    }

    pub fn get_all1(&self, query: Any) -> IDBRequest {
        self.inner
            .call("getAll", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn get_all2(&self, query: Any, count: u32) -> IDBRequest {
        self.inner
            .call("getAll", &[query.into(), count.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBIndex {
    pub fn get_all_keys0(&self) -> IDBRequest {
        self.inner.call("getAllKeys", &[]).as_::<IDBRequest>()
    }

    pub fn get_all_keys1(&self, query: Any) -> IDBRequest {
        self.inner
            .call("getAllKeys", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn get_all_keys2(&self, query: Any, count: u32) -> IDBRequest {
        self.inner
            .call("getAllKeys", &[query.into(), count.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBIndex {
    pub fn count0(&self) -> IDBRequest {
        self.inner.call("count", &[]).as_::<IDBRequest>()
    }

    pub fn count1(&self, query: Any) -> IDBRequest {
        self.inner
            .call("count", &[query.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBIndex {
    pub fn open_cursor0(&self) -> IDBRequest {
        self.inner.call("openCursor", &[]).as_::<IDBRequest>()
    }

    pub fn open_cursor1(&self, query: Any) -> IDBRequest {
        self.inner
            .call("openCursor", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn open_cursor2(&self, query: Any, direction: IDBCursorDirection) -> IDBRequest {
        self.inner
            .call("openCursor", &[query.into(), direction.into()])
            .as_::<IDBRequest>()
    }
}
impl IDBIndex {
    pub fn open_key_cursor0(&self) -> IDBRequest {
        self.inner.call("openKeyCursor", &[]).as_::<IDBRequest>()
    }

    pub fn open_key_cursor1(&self, query: Any) -> IDBRequest {
        self.inner
            .call("openKeyCursor", &[query.into()])
            .as_::<IDBRequest>()
    }

    pub fn open_key_cursor2(&self, query: Any, direction: IDBCursorDirection) -> IDBRequest {
        self.inner
            .call("openKeyCursor", &[query.into(), direction.into()])
            .as_::<IDBRequest>()
    }
}
