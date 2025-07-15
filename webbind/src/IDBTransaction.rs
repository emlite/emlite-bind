use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBTransaction {
    inner: EventTarget,
}
impl FromVal for IDBTransaction {
    fn from_val(v: &emlite::Val) -> Self {
        IDBTransaction { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBTransaction {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBTransaction {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBTransaction {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<IDBTransaction> for emlite::Val {
    fn from(s: IDBTransaction) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IDBTransaction);


impl IDBTransaction {
    pub fn object_store_names(&self) -> DOMStringList {
        self.inner.get("objectStoreNames").as_::<DOMStringList>()
    }

}
impl IDBTransaction {
    pub fn mode(&self) -> IDBTransactionMode {
        self.inner.get("mode").as_::<IDBTransactionMode>()
    }

}
impl IDBTransaction {
    pub fn durability(&self) -> IDBTransactionDurability {
        self.inner.get("durability").as_::<IDBTransactionDurability>()
    }

}
impl IDBTransaction {
    pub fn db(&self) -> IDBDatabase {
        self.inner.get("db").as_::<IDBDatabase>()
    }

}
impl IDBTransaction {
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }

}
impl IDBTransaction {
    pub fn object_store(&self, name: DOMString) -> IDBObjectStore {
        self.inner.call("objectStore", &[name.into(), ]).as_::<IDBObjectStore>()
    }

}
impl IDBTransaction {
    pub fn commit(&self, ) -> Undefined {
        self.inner.call("commit", &[]).as_::<Undefined>()
    }

}
impl IDBTransaction {
    pub fn abort(&self, ) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }

}
impl IDBTransaction {
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    pub fn set_onabort(&mut self, value: Any) {
        self.inner.set("onabort", value);
    }

}
impl IDBTransaction {
    pub fn oncomplete(&self) -> Any {
        self.inner.get("oncomplete").as_::<Any>()
    }

    pub fn set_oncomplete(&mut self, value: Any) {
        self.inner.set("oncomplete", value);
    }

}
impl IDBTransaction {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
        self.inner.set("onerror", value);
    }

}
