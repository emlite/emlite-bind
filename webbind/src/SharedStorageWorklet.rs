use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SharedStorageUrlWithMetadata {
    inner: emlite::Val,
}
impl FromVal for SharedStorageUrlWithMetadata {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageUrlWithMetadata { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageUrlWithMetadata {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageUrlWithMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageUrlWithMetadata> for emlite::Val {
    fn from(s: SharedStorageUrlWithMetadata) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageUrlWithMetadata {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }

    pub fn set_url(&mut self, value: jsbind::USVString) {
        self.inner.set("url", value);
    }
}
impl SharedStorageUrlWithMetadata {
    pub fn reporting_metadata(&self) -> jsbind::Object {
        self.inner.get("reportingMetadata").as_::<jsbind::Object>()
    }

    pub fn set_reporting_metadata(&mut self, value: jsbind::Object) {
        self.inner.set("reportingMetadata", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SharedStorageRunOperationMethodOptions {
    inner: emlite::Val,
}
impl FromVal for SharedStorageRunOperationMethodOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageRunOperationMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageRunOperationMethodOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageRunOperationMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SharedStorageRunOperationMethodOptions> for emlite::Val {
    fn from(s: SharedStorageRunOperationMethodOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageRunOperationMethodOptions {
    pub fn data(&self) -> jsbind::Object {
        self.inner.get("data").as_::<jsbind::Object>()
    }

    pub fn set_data(&mut self, value: jsbind::Object) {
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
    pub fn private_aggregation_config(&self) -> jsbind::Any {
        self.inner
            .get("privateAggregationConfig")
            .as_::<jsbind::Any>()
    }

    pub fn set_private_aggregation_config(&mut self, value: jsbind::Any) {
        self.inner.set("privateAggregationConfig", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    pub fn saved_query(&self) -> jsbind::DOMString {
        self.inner.get("savedQuery").as_::<jsbind::DOMString>()
    }

    pub fn set_saved_query(&mut self, value: jsbind::DOMString) {
        self.inner.set("savedQuery", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SharedStorageWorklet {
    inner: Worklet,
}
impl FromVal for SharedStorageWorklet {
    fn from_val(v: &emlite::Val) -> Self {
        SharedStorageWorklet {
            inner: Worklet::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<SharedStorageWorklet> for emlite::Val {
    fn from(s: SharedStorageWorklet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SharedStorageWorklet {
    pub fn select_url0(
        &self,
        name: jsbind::DOMString,
        urls: jsbind::Sequence<SharedStorageUrlWithMetadata>,
    ) -> jsbind::Promise {
        self.inner
            .call("selectURL", &[name.into(), urls.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn select_url1(
        &self,
        name: jsbind::DOMString,
        urls: jsbind::Sequence<SharedStorageUrlWithMetadata>,
        options: SharedStorageRunOperationMethodOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("selectURL", &[name.into(), urls.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SharedStorageWorklet {
    pub fn run0(&self, name: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("run", &[name.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn run1(
        &self,
        name: jsbind::DOMString,
        options: SharedStorageRunOperationMethodOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("run", &[name.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
