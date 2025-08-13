use super::*;




/// The IDBTransaction class.
/// [`IDBTransaction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBTransaction {
    inner: EventTarget,
}

impl FromVal for IDBTransaction {
    fn from_val(v: &Any) -> Self {
        IDBTransaction { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for IDBTransaction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBTransaction {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBTransaction> for Any {
    fn from(s: IDBTransaction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBTransaction> for Any {
    fn from(s: &IDBTransaction) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBTransaction);


impl IDBTransaction {
    /// Getter of the `objectStoreNames` attribute.
    /// [`IDBTransaction.objectStoreNames`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames)
    pub fn object_store_names(&self) -> DOMStringList {
        self.inner.get("objectStoreNames").as_::<DOMStringList>()
    }

}
impl IDBTransaction {
    /// Getter of the `mode` attribute.
    /// [`IDBTransaction.mode`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode)
    pub fn mode(&self) -> IDBTransactionMode {
        self.inner.get("mode").as_::<IDBTransactionMode>()
    }

}
impl IDBTransaction {
    /// Getter of the `durability` attribute.
    /// [`IDBTransaction.durability`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/durability)
    pub fn durability(&self) -> IDBTransactionDurability {
        self.inner.get("durability").as_::<IDBTransactionDurability>()
    }

}
impl IDBTransaction {
    /// Getter of the `db` attribute.
    /// [`IDBTransaction.db`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)
    pub fn db(&self) -> IDBDatabase {
        self.inner.get("db").as_::<IDBDatabase>()
    }

}
impl IDBTransaction {
    /// Getter of the `error` attribute.
    /// [`IDBTransaction.error`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error)
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }

}
impl IDBTransaction {
    /// The objectStore method.
    /// [`IDBTransaction.objectStore`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore)
    pub fn object_store(&self, name: &JsString) -> IDBObjectStore {
        self.inner.call("objectStore", &[name.into(), ]).as_::<IDBObjectStore>()
    }
}
impl IDBTransaction {
    /// The commit method.
    /// [`IDBTransaction.commit`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/commit)
    pub fn commit(&self, ) -> Undefined {
        self.inner.call("commit", &[]).as_::<Undefined>()
    }
}
impl IDBTransaction {
    /// The abort method.
    /// [`IDBTransaction.abort`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort)
    pub fn abort(&self, ) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl IDBTransaction {
    /// Getter of the `onabort` attribute.
    /// [`IDBTransaction.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    /// Setter of the `onabort` attribute.
    /// [`IDBTransaction.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)
    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
impl IDBTransaction {
    /// Getter of the `oncomplete` attribute.
    /// [`IDBTransaction.oncomplete`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)
    pub fn oncomplete(&self) -> Any {
        self.inner.get("oncomplete").as_::<Any>()
    }

    /// Setter of the `oncomplete` attribute.
    /// [`IDBTransaction.oncomplete`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)
    pub fn set_oncomplete(&mut self, value: &Any) {
        self.inner.set("oncomplete", value);
    }
}
impl IDBTransaction {
    /// Getter of the `onerror` attribute.
    /// [`IDBTransaction.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`IDBTransaction.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
