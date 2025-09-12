use super::*;

/// The SharedStorage class.
/// [`SharedStorage`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorage {
    inner: Any,
}

impl FromVal for SharedStorage {
    fn from_val(v: &Any) -> Self {
        SharedStorage {
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

impl core::ops::Deref for SharedStorage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedStorage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedStorage {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SharedStorage> for Any {
    fn from(s: SharedStorage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedStorage> for Any {
    fn from(s: &SharedStorage) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SharedStorage);

impl SharedStorage {
    /// Getter of the `worklet` attribute.
    /// [`SharedStorage.worklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/worklet)
    pub fn worklet(&self) -> SharedStorageWorklet {
        self.inner.get("worklet").as_::<SharedStorageWorklet>()
    }
}
impl SharedStorage {
    /// The get method.
    /// [`SharedStorage.get`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/get)
    pub fn get(&self, key: &JsString) -> Promise<JsString> {
        self.inner
            .call("get", &[key.into()])
            .as_::<Promise<JsString>>()
    }
}
impl SharedStorage {
    /// The set method.
    /// [`SharedStorage.set`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/set)
    pub fn set0(&self, key: &JsString, value: &JsString) -> Promise<Any> {
        self.inner
            .call("set", &[key.into(), value.into()])
            .as_::<Promise<Any>>()
    }
    /// The set method.
    /// [`SharedStorage.set`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/set)
    pub fn set1(
        &self,
        key: &JsString,
        value: &JsString,
        options: &SharedStorageSetMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("set", &[key.into(), value.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The append method.
    /// [`SharedStorage.append`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/append)
    pub fn append0(&self, key: &JsString, value: &JsString) -> Promise<Any> {
        self.inner
            .call("append", &[key.into(), value.into()])
            .as_::<Promise<Any>>()
    }
    /// The append method.
    /// [`SharedStorage.append`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/append)
    pub fn append1(
        &self,
        key: &JsString,
        value: &JsString,
        options: &SharedStorageModifierMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("append", &[key.into(), value.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The delete method.
    /// [`SharedStorage.delete`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/delete)
    pub fn delete0(&self, key: &JsString) -> Promise<Any> {
        self.inner
            .call("delete", &[key.into()])
            .as_::<Promise<Any>>()
    }
    /// The delete method.
    /// [`SharedStorage.delete`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/delete)
    pub fn delete1(
        &self,
        key: &JsString,
        options: &SharedStorageModifierMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("delete", &[key.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The clear method.
    /// [`SharedStorage.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/clear)
    pub fn clear0(&self) -> Promise<Any> {
        self.inner.call("clear", &[]).as_::<Promise<Any>>()
    }
    /// The clear method.
    /// [`SharedStorage.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/clear)
    pub fn clear1(&self, options: &SharedStorageModifierMethodOptions) -> Promise<Any> {
        self.inner
            .call("clear", &[options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The batchUpdate method.
    /// [`SharedStorage.batchUpdate`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/batchUpdate)
    pub fn batch_update0(&self, methods: &TypedArray<SharedStorageModifierMethod>) -> Promise<Any> {
        self.inner
            .call("batchUpdate", &[methods.into()])
            .as_::<Promise<Any>>()
    }
    /// The batchUpdate method.
    /// [`SharedStorage.batchUpdate`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/batchUpdate)
    pub fn batch_update1(
        &self,
        methods: &TypedArray<SharedStorageModifierMethod>,
        options: &SharedStorageModifierMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("batchUpdate", &[methods.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The selectURL method.
    /// [`SharedStorage.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/selectURL)
    pub fn select_url0(
        &self,
        name: &JsString,
        urls: &TypedArray<SharedStorageUrlWithMetadata>,
    ) -> Promise<Any> {
        self.inner
            .call("selectURL", &[name.into(), urls.into()])
            .as_::<Promise<Any>>()
    }
    /// The selectURL method.
    /// [`SharedStorage.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/selectURL)
    pub fn select_url1(
        &self,
        name: &JsString,
        urls: &TypedArray<SharedStorageUrlWithMetadata>,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("selectURL", &[name.into(), urls.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The run method.
    /// [`SharedStorage.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/run)
    pub fn run0(&self, name: &JsString) -> Promise<Any> {
        self.inner.call("run", &[name.into()]).as_::<Promise<Any>>()
    }
    /// The run method.
    /// [`SharedStorage.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/run)
    pub fn run1(
        &self,
        name: &JsString,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("run", &[name.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorage {
    /// The createWorklet method.
    /// [`SharedStorage.createWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/createWorklet)
    pub fn create_worklet0(&self, module_url: &JsString) -> Promise<SharedStorageWorklet> {
        self.inner
            .call("createWorklet", &[module_url.into()])
            .as_::<Promise<SharedStorageWorklet>>()
    }
    /// The createWorklet method.
    /// [`SharedStorage.createWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/createWorklet)
    pub fn create_worklet1(
        &self,
        module_url: &JsString,
        options: &SharedStorageWorkletOptions,
    ) -> Promise<SharedStorageWorklet> {
        self.inner
            .call("createWorklet", &[module_url.into(), options.into()])
            .as_::<Promise<SharedStorageWorklet>>()
    }
}
impl SharedStorage {
    /// The length method.
    /// [`SharedStorage.length`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/length)
    pub fn length(&self) -> Promise<u32> {
        self.inner.call("length", &[]).as_::<Promise<u32>>()
    }
}
impl SharedStorage {
    /// The remainingBudget method.
    /// [`SharedStorage.remainingBudget`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/remainingBudget)
    pub fn remaining_budget(&self) -> Promise<f64> {
        self.inner
            .call("remainingBudget", &[])
            .as_::<Promise<f64>>()
    }
}
