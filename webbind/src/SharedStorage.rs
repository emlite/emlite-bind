use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageSetMethodOptions {
    inner: emlite::Val,
}
impl FromVal for SharedStorageSetMethodOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageSetMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageSetMethodOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageSetMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedStorageSetMethodOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorageSetMethodOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorageSetMethodOptions> for emlite::Val {
    fn from(s: SharedStorageSetMethodOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedStorageSetMethodOptions> for emlite::Val {
    fn from(s: &SharedStorageSetMethodOptions) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for SharedStorageModifierMethodOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageModifierMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageModifierMethodOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageModifierMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedStorageModifierMethodOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorageModifierMethodOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorageModifierMethodOptions> for emlite::Val {
    fn from(s: SharedStorageModifierMethodOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedStorageModifierMethodOptions> for emlite::Val {
    fn from(s: &SharedStorageModifierMethodOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl SharedStorageModifierMethodOptions {
    pub fn with_lock(&self) -> DOMString {
        self.inner.get("withLock").as_::<DOMString>()
    }

    pub fn set_with_lock(&mut self, value: DOMString) {
        self.inner.set("withLock", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorkletOptions {
    inner: emlite::Val,
}
impl FromVal for SharedStorageWorkletOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageWorkletOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageWorkletOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageWorkletOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedStorageWorkletOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorageWorkletOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorageWorkletOptions> for emlite::Val {
    fn from(s: SharedStorageWorkletOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedStorageWorkletOptions> for emlite::Val {
    fn from(s: &SharedStorageWorkletOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl SharedStorageWorkletOptions {
    pub fn data_origin(&self) -> USVString {
        self.inner.get("dataOrigin").as_::<USVString>()
    }

    pub fn set_data_origin(&mut self, value: USVString) {
        self.inner.set("dataOrigin", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorage {
    inner: emlite::Val,
}
impl FromVal for SharedStorage {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorage {
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
impl core::ops::Deref for SharedStorage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedStorage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedStorage {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedStorage> for emlite::Val {
    fn from(s: SharedStorage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedStorage> for emlite::Val {
    fn from(s: &SharedStorage) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorage);

impl SharedStorage {
    pub fn get(&self, key: DOMString) -> Promise {
        self.inner.call("get", &[key.into()]).as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn set0(&self, key: DOMString, value: DOMString) -> Promise {
        self.inner
            .call("set", &[key.into(), value.into()])
            .as_::<Promise>()
    }

    pub fn set1(
        &self,
        key: DOMString,
        value: DOMString,
        options: SharedStorageSetMethodOptions,
    ) -> Promise {
        self.inner
            .call("set", &[key.into(), value.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn append0(&self, key: DOMString, value: DOMString) -> Promise {
        self.inner
            .call("append", &[key.into(), value.into()])
            .as_::<Promise>()
    }

    pub fn append1(
        &self,
        key: DOMString,
        value: DOMString,
        options: SharedStorageModifierMethodOptions,
    ) -> Promise {
        self.inner
            .call("append", &[key.into(), value.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn delete0(&self, key: DOMString) -> Promise {
        self.inner.call("delete", &[key.into()]).as_::<Promise>()
    }

    pub fn delete1(&self, key: DOMString, options: SharedStorageModifierMethodOptions) -> Promise {
        self.inner
            .call("delete", &[key.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn clear0(&self) -> Promise {
        self.inner.call("clear", &[]).as_::<Promise>()
    }

    pub fn clear1(&self, options: SharedStorageModifierMethodOptions) -> Promise {
        self.inner.call("clear", &[options.into()]).as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn batch_update0(&self, methods: Sequence<SharedStorageModifierMethod>) -> Promise {
        self.inner
            .call("batchUpdate", &[methods.into()])
            .as_::<Promise>()
    }

    pub fn batch_update1(
        &self,
        methods: Sequence<SharedStorageModifierMethod>,
        options: SharedStorageModifierMethodOptions,
    ) -> Promise {
        self.inner
            .call("batchUpdate", &[methods.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn select_url0(
        &self,
        name: DOMString,
        urls: Sequence<SharedStorageUrlWithMetadata>,
    ) -> Promise {
        self.inner
            .call("selectURL", &[name.into(), urls.into()])
            .as_::<Promise>()
    }

    pub fn select_url1(
        &self,
        name: DOMString,
        urls: Sequence<SharedStorageUrlWithMetadata>,
        options: SharedStorageRunOperationMethodOptions,
    ) -> Promise {
        self.inner
            .call("selectURL", &[name.into(), urls.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn run0(&self, name: DOMString) -> Promise {
        self.inner.call("run", &[name.into()]).as_::<Promise>()
    }

    pub fn run1(
        &self,
        name: DOMString,
        options: SharedStorageRunOperationMethodOptions,
    ) -> Promise {
        self.inner
            .call("run", &[name.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn create_worklet0(&self, module_url: USVString) -> Promise {
        self.inner
            .call("createWorklet", &[module_url.into()])
            .as_::<Promise>()
    }

    pub fn create_worklet1(
        &self,
        module_url: USVString,
        options: SharedStorageWorkletOptions,
    ) -> Promise {
        self.inner
            .call("createWorklet", &[module_url.into(), options.into()])
            .as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn worklet(&self) -> SharedStorageWorklet {
        self.inner.get("worklet").as_::<SharedStorageWorklet>()
    }
}
impl SharedStorage {
    pub fn length(&self) -> Promise {
        self.inner.call("length", &[]).as_::<Promise>()
    }
}
impl SharedStorage {
    pub fn remaining_budget(&self) -> Promise {
        self.inner.call("remainingBudget", &[]).as_::<Promise>()
    }
}
