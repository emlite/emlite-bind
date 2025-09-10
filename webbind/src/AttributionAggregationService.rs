use super::*;

/// The AttributionAggregationService dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionAggregationService {
    inner: Any,
}

impl FromVal for AttributionAggregationService {
    fn from_val(v: &Any) -> Self {
        AttributionAggregationService { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AttributionAggregationService {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AttributionAggregationService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AttributionAggregationService {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AttributionAggregationService {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AttributionAggregationService> for Any {
    fn from(s: AttributionAggregationService) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AttributionAggregationService> for Any {
    fn from(s: &AttributionAggregationService) -> Any {
        s.inner.clone()
    }
}

impl AttributionAggregationService {
    /// Getter of the `protocol` attribute.
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
