use super::*;

/// The AttributionAggregationServices class.
/// [`AttributionAggregationServices`](https://developer.mozilla.org/en-US/docs/Web/API/AttributionAggregationServices)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionAggregationServices {
    inner: Any,
}
impl FromVal for AttributionAggregationServices {
    fn from_val(v: &Any) -> Self {
        AttributionAggregationServices {
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
impl core::ops::Deref for AttributionAggregationServices {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionAggregationServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AttributionAggregationServices {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AttributionAggregationServices {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AttributionAggregationServices> for Any {
    fn from(s: AttributionAggregationServices) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AttributionAggregationServices> for Any {
    fn from(s: &AttributionAggregationServices) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AttributionAggregationServices);
