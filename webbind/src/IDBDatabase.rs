use super::*;

/// The IDBDatabase class.
/// [`IDBDatabase`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBDatabase {
    inner: EventTarget,
}

impl FromVal for IDBDatabase {
    fn from_val(v: &Any) -> Self {
        IDBDatabase {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBDatabase {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBDatabase {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBDatabase {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBDatabase> for Any {
    fn from(s: IDBDatabase) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBDatabase> for Any {
    fn from(s: &IDBDatabase) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBDatabase);

impl IDBDatabase {
    /// Getter of the `name` attribute.
    /// [`IDBDatabase.name`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl IDBDatabase {
    /// Getter of the `version` attribute.
    /// [`IDBDatabase.version`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)
    pub fn version(&self) -> u64 {
        self.inner.get("version").as_::<u64>()
    }
}
impl IDBDatabase {
    /// Getter of the `objectStoreNames` attribute.
    /// [`IDBDatabase.objectStoreNames`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)
    pub fn object_store_names(&self) -> DOMStringList {
        self.inner.get("objectStoreNames").as_::<DOMStringList>()
    }
}
impl IDBDatabase {
    /// Getter of the `onabort` attribute.
    /// [`IDBDatabase.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    /// Setter of the `onabort` attribute.
    /// [`IDBDatabase.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)
    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
impl IDBDatabase {
    /// Getter of the `onclose` attribute.
    /// [`IDBDatabase.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`IDBDatabase.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
impl IDBDatabase {
    /// Getter of the `onerror` attribute.
    /// [`IDBDatabase.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`IDBDatabase.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl IDBDatabase {
    /// Getter of the `onversionchange` attribute.
    /// [`IDBDatabase.onversionchange`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)
    pub fn onversionchange(&self) -> Any {
        self.inner.get("onversionchange").as_::<Any>()
    }

    /// Setter of the `onversionchange` attribute.
    /// [`IDBDatabase.onversionchange`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)
    pub fn set_onversionchange(&mut self, value: &Any) {
        self.inner.set("onversionchange", value);
    }
}
impl IDBDatabase {
    /// The transaction method.
    /// [`IDBDatabase.transaction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    pub fn transaction0(&self, store_names: &Any) -> IDBTransaction {
        self.inner
            .call("transaction", &[store_names.into()])
            .as_::<IDBTransaction>()
    }
    /// The transaction method.
    /// [`IDBDatabase.transaction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    pub fn transaction1(&self, store_names: &Any, mode: &IDBTransactionMode) -> IDBTransaction {
        self.inner
            .call("transaction", &[store_names.into(), mode.into()])
            .as_::<IDBTransaction>()
    }
    /// The transaction method.
    /// [`IDBDatabase.transaction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)
    pub fn transaction2(
        &self,
        store_names: &Any,
        mode: &IDBTransactionMode,
        options: &IDBTransactionOptions,
    ) -> IDBTransaction {
        self.inner
            .call(
                "transaction",
                &[store_names.into(), mode.into(), options.into()],
            )
            .as_::<IDBTransaction>()
    }
}
impl IDBDatabase {
    /// The close method.
    /// [`IDBDatabase.close`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl IDBDatabase {
    /// The createObjectStore method.
    /// [`IDBDatabase.createObjectStore`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)
    pub fn create_object_store0(&self, name: &JsString) -> IDBObjectStore {
        self.inner
            .call("createObjectStore", &[name.into()])
            .as_::<IDBObjectStore>()
    }
    /// The createObjectStore method.
    /// [`IDBDatabase.createObjectStore`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)
    pub fn create_object_store1(
        &self,
        name: &JsString,
        options: &IDBObjectStoreParameters,
    ) -> IDBObjectStore {
        self.inner
            .call("createObjectStore", &[name.into(), options.into()])
            .as_::<IDBObjectStore>()
    }
}
impl IDBDatabase {
    /// The deleteObjectStore method.
    /// [`IDBDatabase.deleteObjectStore`](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)
    pub fn delete_object_store(&self, name: &JsString) -> Undefined {
        self.inner
            .call("deleteObjectStore", &[name.into()])
            .as_::<Undefined>()
    }
}
