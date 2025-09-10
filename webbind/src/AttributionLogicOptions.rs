use super::*;

/// The AttributionLogicOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionLogicOptions {
    inner: Any,
}

impl FromVal for AttributionLogicOptions {
    fn from_val(v: &Any) -> Self {
        AttributionLogicOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AttributionLogicOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AttributionLogicOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AttributionLogicOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AttributionLogicOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AttributionLogicOptions> for Any {
    fn from(s: AttributionLogicOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AttributionLogicOptions> for Any {
    fn from(s: &AttributionLogicOptions) -> Any {
        s.inner.clone()
    }
}

impl AttributionLogicOptions {
    /// Getter of the `credit` attribute.
    pub fn credit(&self) -> TypedArray<f64> {
        self.inner.get("credit").as_::<TypedArray<f64>>()
    }

    /// Setter of the `credit` attribute.
    pub fn set_credit(&mut self, value: TypedArray<f64>) {
        self.inner.set("credit", value);
    }
}
