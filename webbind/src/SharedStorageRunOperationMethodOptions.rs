use super::*;




/// The SharedStorageRunOperationMethodOptions dictionary.
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
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Object {
        self.inner.get("data").as_::<Object>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Object) {
        self.inner.set("data", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    /// Getter of the `resolveToConfig` attribute.
    pub fn resolve_to_config(&self) -> bool {
        self.inner.get("resolveToConfig").as_::<bool>()
    }

    /// Setter of the `resolveToConfig` attribute.
    pub fn set_resolve_to_config(&mut self, value: bool) {
        self.inner.set("resolveToConfig", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    /// Getter of the `keepAlive` attribute.
    pub fn keep_alive(&self) -> bool {
        self.inner.get("keepAlive").as_::<bool>()
    }

    /// Setter of the `keepAlive` attribute.
    pub fn set_keep_alive(&mut self, value: bool) {
        self.inner.set("keepAlive", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    /// Getter of the `privateAggregationConfig` attribute.
    pub fn private_aggregation_config(&self) -> SharedStoragePrivateAggregationConfig {
        self.inner.get("privateAggregationConfig").as_::<SharedStoragePrivateAggregationConfig>()
    }

    /// Setter of the `privateAggregationConfig` attribute.
    pub fn set_private_aggregation_config(&mut self, value: &SharedStoragePrivateAggregationConfig) {
        self.inner.set("privateAggregationConfig", value);
    }
}
impl SharedStorageRunOperationMethodOptions {
    /// Getter of the `savedQuery` attribute.
    pub fn saved_query(&self) -> JsString {
        self.inner.get("savedQuery").as_::<JsString>()
    }

    /// Setter of the `savedQuery` attribute.
    pub fn set_saved_query(&mut self, value: &JsString) {
        self.inner.set("savedQuery", value);
    }
}
