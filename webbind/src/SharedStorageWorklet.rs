use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageUrlWithMetadata {
    inner: Any,
}
impl FromVal for SharedStorageUrlWithMetadata {
    fn from_val(v: &Any) -> Self {
        SharedStorageUrlWithMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageUrlWithMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageUrlWithMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageUrlWithMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageUrlWithMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageUrlWithMetadata> for Any {
    fn from(s: SharedStorageUrlWithMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageUrlWithMetadata> for Any {
    fn from(s: &SharedStorageUrlWithMetadata) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageUrlWithMetadata {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }

    pub fn set_url(&mut self, value: &str) {
        self.inner.set("url", value);
    }
}
impl SharedStorageUrlWithMetadata {
    pub fn reporting_metadata(&self) -> Object {
        self.inner.get("reportingMetadata").as_::<Object>()
    }

    pub fn set_reporting_metadata(&mut self, value: &Object) {
        self.inner.set("reportingMetadata", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageRunOperationMethodOptions {
    inner: Any,
}
impl FromVal for SharedStorageRunOperationMethodOptions {
    fn from_val(v: &Any) -> Self {
        SharedStorageRunOperationMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageRunOperationMethodOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageRunOperationMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageRunOperationMethodOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageRunOperationMethodOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageRunOperationMethodOptions> for Any {
    fn from(s: SharedStorageRunOperationMethodOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageRunOperationMethodOptions> for Any {
    fn from(s: &SharedStorageRunOperationMethodOptions) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageRunOperationMethodOptions {
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    pub fn resolve_to_config(&self) -> bool {
        self.inner.get("resolveToConfig").as_::<bool>()
    }

    pub fn set_resolve_to_config(&mut self, value: bool) {
        self.inner.set("resolveToConfig", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    pub fn keep_alive(&self) -> bool {
        self.inner.get("keepAlive").as_::<bool>()
    }

    pub fn set_keep_alive(&mut self, value: bool) {
        self.inner.set("keepAlive", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    pub fn private_aggregation_config(&self) -> Any {
        self.inner.get("privateAggregationConfig").as_::<Any>()
    }

    pub fn set_private_aggregation_config(&mut self, value: &Any) {
        self.inner.set("privateAggregationConfig", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    pub fn saved_query(&self) -> String {
        self.inner.get("savedQuery").as_::<String>()
    }

    pub fn set_saved_query(&mut self, value: &str) {
        self.inner.set("savedQuery", value);
    }
}
/// The SharedStorageWorklet class.
/// [`SharedStorageWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageWorklet {
    inner: Worklet,
}
impl FromVal for SharedStorageWorklet {
    fn from_val(v: &Any) -> Self {
        SharedStorageWorklet {
            inner: Worklet::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageWorklet {
    type Target = Worklet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageWorklet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageWorklet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageWorklet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageWorklet> for Any {
    fn from(s: SharedStorageWorklet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageWorklet> for Any {
    fn from(s: &SharedStorageWorklet) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageWorklet);

impl SharedStorageWorklet {
    /// The selectURL method.
    /// [`SharedStorageWorklet.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/selectURL)
    pub fn select_url0(
        &self,
        name: &str,
        urls: &Sequence<SharedStorageUrlWithMetadata>,
    ) -> Promise<Any> {
        self.inner
            .call("selectURL", &[name.into(), urls.into()])
            .as_::<Promise<Any>>()
    }
    /// The selectURL method.
    /// [`SharedStorageWorklet.selectURL`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/selectURL)
    pub fn select_url1(
        &self,
        name: &str,
        urls: &Sequence<SharedStorageUrlWithMetadata>,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("selectURL", &[name.into(), urls.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl SharedStorageWorklet {
    /// The run method.
    /// [`SharedStorageWorklet.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/run)
    pub fn run0(&self, name: &str) -> Promise<Any> {
        self.inner.call("run", &[name.into()]).as_::<Promise<Any>>()
    }
    /// The run method.
    /// [`SharedStorageWorklet.run`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageWorklet/run)
    pub fn run1(
        &self,
        name: &str,
        options: &SharedStorageRunOperationMethodOptions,
    ) -> Promise<Any> {
        self.inner
            .call("run", &[name.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
