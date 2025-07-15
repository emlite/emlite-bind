use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBTransactionOptions {
    inner: emlite::Val,
}
impl FromVal for IDBTransactionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        IDBTransactionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBTransactionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBTransactionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBTransactionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBTransactionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBTransactionOptions> for emlite::Val {
    fn from(s: IDBTransactionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBTransactionOptions {
    pub fn durability(&self) -> IDBTransactionDurability {
        self.inner
            .get("durability")
            .as_::<IDBTransactionDurability>()
    }

    pub fn set_durability(&mut self, value: IDBTransactionDurability) {
        self.inner.set("durability", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBObjectStoreParameters {
    inner: emlite::Val,
}
impl FromVal for IDBObjectStoreParameters {
    fn from_val(v: &emlite::Val) -> Self {
        IDBObjectStoreParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBObjectStoreParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBObjectStoreParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IDBObjectStoreParameters {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBObjectStoreParameters {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBObjectStoreParameters> for emlite::Val {
    fn from(s: IDBObjectStoreParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBObjectStoreParameters {
    pub fn key_path(&self) -> Any {
        self.inner.get("keyPath").as_::<Any>()
    }

    pub fn set_key_path(&mut self, value: Any) {
        self.inner.set("keyPath", value);
    }
}
impl IDBObjectStoreParameters {
    pub fn auto_increment(&self) -> bool {
        self.inner.get("autoIncrement").as_::<bool>()
    }

    pub fn set_auto_increment(&mut self, value: bool) {
        self.inner.set("autoIncrement", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBDatabase {
    inner: EventTarget,
}
impl FromVal for IDBDatabase {
    fn from_val(v: &emlite::Val) -> Self {
        IDBDatabase {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for IDBDatabase {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBDatabase {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IDBDatabase> for emlite::Val {
    fn from(s: IDBDatabase) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IDBDatabase);

impl IDBDatabase {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl IDBDatabase {
    pub fn version(&self) -> u64 {
        self.inner.get("version").as_::<u64>()
    }
}
impl IDBDatabase {
    pub fn object_store_names(&self) -> DOMStringList {
        self.inner.get("objectStoreNames").as_::<DOMStringList>()
    }
}
impl IDBDatabase {
    pub fn transaction0(&self, store_names: Any) -> IDBTransaction {
        self.inner
            .call("transaction", &[store_names.into()])
            .as_::<IDBTransaction>()
    }

    pub fn transaction1(&self, store_names: Any, mode: IDBTransactionMode) -> IDBTransaction {
        self.inner
            .call("transaction", &[store_names.into(), mode.into()])
            .as_::<IDBTransaction>()
    }

    pub fn transaction2(
        &self,
        store_names: Any,
        mode: IDBTransactionMode,
        options: IDBTransactionOptions,
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
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl IDBDatabase {
    pub fn create_object_store0(&self, name: DOMString) -> IDBObjectStore {
        self.inner
            .call("createObjectStore", &[name.into()])
            .as_::<IDBObjectStore>()
    }

    pub fn create_object_store1(
        &self,
        name: DOMString,
        options: IDBObjectStoreParameters,
    ) -> IDBObjectStore {
        self.inner
            .call("createObjectStore", &[name.into(), options.into()])
            .as_::<IDBObjectStore>()
    }
}
impl IDBDatabase {
    pub fn delete_object_store(&self, name: DOMString) -> Undefined {
        self.inner
            .call("deleteObjectStore", &[name.into()])
            .as_::<Undefined>()
    }
}
impl IDBDatabase {
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    pub fn set_onabort(&mut self, value: Any) {
        self.inner.set("onabort", value);
    }
}
impl IDBDatabase {
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    pub fn set_onclose(&mut self, value: Any) {
        self.inner.set("onclose", value);
    }
}
impl IDBDatabase {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
        self.inner.set("onerror", value);
    }
}
impl IDBDatabase {
    pub fn onversionchange(&self) -> Any {
        self.inner.get("onversionchange").as_::<Any>()
    }

    pub fn set_onversionchange(&mut self, value: Any) {
        self.inner.set("onversionchange", value);
    }
}
