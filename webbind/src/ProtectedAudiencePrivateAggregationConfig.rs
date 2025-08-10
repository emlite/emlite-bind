use super::*;

/// The ProtectedAudiencePrivateAggregationConfig dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProtectedAudiencePrivateAggregationConfig {
    inner: Any,
}

impl FromVal for ProtectedAudiencePrivateAggregationConfig {
    fn from_val(v: &Any) -> Self {
        ProtectedAudiencePrivateAggregationConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProtectedAudiencePrivateAggregationConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProtectedAudiencePrivateAggregationConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProtectedAudiencePrivateAggregationConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProtectedAudiencePrivateAggregationConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ProtectedAudiencePrivateAggregationConfig> for Any {
    fn from(s: ProtectedAudiencePrivateAggregationConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProtectedAudiencePrivateAggregationConfig> for Any {
    fn from(s: &ProtectedAudiencePrivateAggregationConfig) -> Any {
        s.inner.clone()
    }
}

impl ProtectedAudiencePrivateAggregationConfig {
    /// Getter of the `aggregationCoordinatorOrigin` attribute.
    pub fn aggregation_coordinator_origin(&self) -> JsString {
        self.inner
            .get("aggregationCoordinatorOrigin")
            .as_::<JsString>()
    }

    /// Setter of the `aggregationCoordinatorOrigin` attribute.
    pub fn set_aggregation_coordinator_origin(&mut self, value: &JsString) {
        self.inner.set("aggregationCoordinatorOrigin", value);
    }
}
