use super::*;

/// The AttributionImpressionResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionImpressionResult {
    inner: Any,
}

impl FromVal for AttributionImpressionResult {
    fn from_val(v: &Any) -> Self {
        AttributionImpressionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AttributionImpressionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AttributionImpressionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AttributionImpressionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AttributionImpressionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AttributionImpressionResult> for Any {
    fn from(s: AttributionImpressionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AttributionImpressionResult> for Any {
    fn from(s: &AttributionImpressionResult) -> Any {
        s.inner.clone()
    }
}
