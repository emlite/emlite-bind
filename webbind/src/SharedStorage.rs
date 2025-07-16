use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageSetMethodOptions {
    inner: Any,
}
impl FromVal for SharedStorageSetMethodOptions {
    fn from_val(v: &Any) -> Self {
        SharedStorageSetMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageSetMethodOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageSetMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageSetMethodOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageSetMethodOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageSetMethodOptions> for Any {
    fn from(s: SharedStorageSetMethodOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageSetMethodOptions> for Any {
    fn from(s: &SharedStorageSetMethodOptions) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageSetMethodOptions {
    pub fn ignore_if_present(&self) -> bool {
        self.inner.get("ignoreIfPresent").as_::<bool>()
    }

    pub fn set_ignore_if_present(&mut self, value: bool) {
        self.inner.set("ignoreIfPresent", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageModifierMethodOptions {
    inner: Any,
}
impl FromVal for SharedStorageModifierMethodOptions {
    fn from_val(v: &Any) -> Self {
        SharedStorageModifierMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageModifierMethodOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageModifierMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageModifierMethodOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageModifierMethodOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageModifierMethodOptions> for Any {
    fn from(s: SharedStorageModifierMethodOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageModifierMethodOptions> for Any {
    fn from(s: &SharedStorageModifierMethodOptions) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageModifierMethodOptions {
    pub fn with_lock(&self) -> String {
        self.inner.get("withLock").as_::<String>()
    }

    pub fn set_with_lock(&mut self, value: &str) {
        self.inner.set("withLock", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorkletOptions {
    inner: Any,
}
impl FromVal for SharedStorageWorkletOptions {
    fn from_val(v: &Any) -> Self {
        SharedStorageWorkletOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageWorkletOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageWorkletOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageWorkletOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageWorkletOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageWorkletOptions> for Any {
    fn from(s: SharedStorageWorkletOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageWorkletOptions> for Any {
    fn from(s: &SharedStorageWorkletOptions) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageWorkletOptions {
    pub fn data_origin(&self) -> String {
        self.inner.get("dataOrigin").as_::<String>()
    }

    pub fn set_data_origin(&mut self, value: &str) {
        self.inner.set("dataOrigin", value);
    }
}
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
    /// The get method.
    /// [`SharedStorage.get`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/get)
    pub fn get(&self, key: &str) -> Promise {
        self.inner.call("get", &[key.into()]).as_::<Promise>()
    }
}
impl SharedStorage {
    /// The set method.
    /// [`SharedStorage.set`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/set)
    pub fn set0(&self, key: &str, value: &str) -> Promise {
        self.inner
            .call("set", &[key.into(), value.into()])
            .as_::<Promise>()
    }
    /// The set method.
    /// [`SharedStorage.set`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/set)
    pub fn set1(&self, key: &str, value: &str, options: &SharedStorageSetMethodOptions) -> Promise {
        self.inner
            .call("set", &[key.into(), value.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// The append method.
    /// [`SharedStorage.append`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/append)
    pub fn append0(&self, key: &str, value: &str) -> Promise {
        self.inner
            .call("append", &[key.into(), value.into()])
            .as_::<Promise>()
    }
    /// The append method.
    /// [`SharedStorage.append`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/append)
    pub fn append1(
        &self,
        key: &str,
        value: &str,
        options: &SharedStorageModifierMethodOptions,
    ) -> Promise {
        self.inner
            .call("append", &[key.into(), value.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// The delete method.
    /// [`SharedStorage.delete`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/delete)
    pub fn delete0(&self, key: &str) -> Promise {
        self.inner.call("delete", &[key.into()]).as_::<Promise>()
    }
    /// The delete method.
    /// [`SharedStorage.delete`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/delete)
    pub fn delete1(&self, key: &str, options: &SharedStorageModifierMethodOptions) -> Promise {
        self.inner
            .call("delete", &[key.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// The clear method.
    /// [`SharedStorage.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/clear)
    pub fn clear0(&self) -> Promise {
        self.inner.call("clear", &[]).as_::<Promise>()
    }
    /// The clear method.
    /// [`SharedStorage.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/clear)
    pub fn clear1(&self, options: &SharedStorageModifierMethodOptions) -> Promise {
        self.inner.call("clear", &[options.into()]).as_::<Promise>()
    }
}
impl SharedStorage {
    /// The batchUpdate method.
    /// [`SharedStorage.batchUpdate`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/batchUpdate)
    pub fn batch_update0(&self, methods: &Sequence<SharedStorageModifierMethod>) -> Promise {
        self.inner
            .call("batchUpdate", &[methods.into()])
            .as_::<Promise>()
    }
    /// The batchUpdate method.
    /// [`SharedStorage.batchUpdate`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/batchUpdate)
    pub fn batch_update1(
        &self,
        methods: &Sequence<SharedStorageModifierMethod>,
        options: &SharedStorageModifierMethodOptions,
    ) -> Promise {
        self.inner
            .call("batchUpdate", &[methods.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// The selectURL method.
    /// [`SharedStorage.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/selectURL)
    pub fn select_url0(
        &self,
        name: &str,
        urls: &Sequence<SharedStorageUrlWithMetadata>,
    ) -> Promise {
        self.inner
            .call("selectURL", &[name.into(), urls.into()])
            .as_::<Promise>()
    }
    /// The selectURL method.
    /// [`SharedStorage.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/selectURL)
    pub fn select_url1(
        &self,
        name: &str,
        urls: &Sequence<SharedStorageUrlWithMetadata>,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise {
        self.inner
            .call("selectURL", &[name.into(), urls.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// The run method.
    /// [`SharedStorage.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/run)
    pub fn run0(&self, name: &str) -> Promise {
        self.inner.call("run", &[name.into()]).as_::<Promise>()
    }
    /// The run method.
    /// [`SharedStorage.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/run)
    pub fn run1(&self, name: &str, options: &SharedStorageRunOperationMethodOptions) -> Promise {
        self.inner
            .call("run", &[name.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// The createWorklet method.
    /// [`SharedStorage.createWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/createWorklet)
    pub fn create_worklet0(&self, module_url: &str) -> Promise {
        self.inner
            .call("createWorklet", &[module_url.into()])
            .as_::<Promise>()
    }
    /// The createWorklet method.
    /// [`SharedStorage.createWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/createWorklet)
    pub fn create_worklet1(
        &self,
        module_url: &str,
        options: &SharedStorageWorkletOptions,
    ) -> Promise {
        self.inner
            .call("createWorklet", &[module_url.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    /// Getter of the `worklet` attribute.
    /// [`SharedStorage.worklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/worklet)
    pub fn worklet(&self) -> SharedStorageWorklet {
        self.inner.get("worklet").as_::<SharedStorageWorklet>()
    }
}
impl SharedStorage {
    /// The length method.
    /// [`SharedStorage.length`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/length)
    pub fn length(&self) -> Promise {
        self.inner.call("length", &[]).as_::<Promise>()
    }
}
impl SharedStorage {
    /// The remainingBudget method.
    /// [`SharedStorage.remainingBudget`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorage/remainingBudget)
    pub fn remaining_budget(&self) -> Promise {
        self.inner.call("remainingBudget", &[]).as_::<Promise>()
    }
}
