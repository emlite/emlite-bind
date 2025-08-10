use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStoragePrivateAggregationConfig {
    inner: Any,
}
impl FromVal for SharedStoragePrivateAggregationConfig {
    fn from_val(v: &Any) -> Self {
        SharedStoragePrivateAggregationConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStoragePrivateAggregationConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStoragePrivateAggregationConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStoragePrivateAggregationConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStoragePrivateAggregationConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStoragePrivateAggregationConfig> for Any {
    fn from(s: SharedStoragePrivateAggregationConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStoragePrivateAggregationConfig> for Any {
    fn from(s: &SharedStoragePrivateAggregationConfig) -> Any {
        s.inner.clone()
    }
}

impl SharedStoragePrivateAggregationConfig {
    pub fn aggregation_coordinator_origin(&self) -> JsString {
        self.inner
            .get("aggregationCoordinatorOrigin")
            .as_::<JsString>()
    }

    pub fn set_aggregation_coordinator_origin(&mut self, value: &JsString) {
        self.inner.set("aggregationCoordinatorOrigin", value);
    }
}
impl SharedStoragePrivateAggregationConfig {
    pub fn context_id(&self) -> JsString {
        self.inner.get("contextId").as_::<JsString>()
    }

    pub fn set_context_id(&mut self, value: &JsString) {
        self.inner.set("contextId", value);
    }
}
impl SharedStoragePrivateAggregationConfig {
    pub fn filtering_id_max_bytes(&self) -> u64 {
        self.inner.get("filteringIdMaxBytes").as_::<u64>()
    }

    pub fn set_filtering_id_max_bytes(&mut self, value: u64) {
        self.inner.set("filteringIdMaxBytes", value);
    }
}
impl SharedStoragePrivateAggregationConfig {
    pub fn max_contributions(&self) -> u64 {
        self.inner.get("maxContributions").as_::<u64>()
    }

    pub fn set_max_contributions(&mut self, value: u64) {
        self.inner.set("maxContributions", value);
    }
}
