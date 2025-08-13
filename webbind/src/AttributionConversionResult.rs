use super::*;




/// The AttributionConversionResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionConversionResult {
    inner: Any,
}

impl FromVal for AttributionConversionResult {
    fn from_val(v: &Any) -> Self {
        AttributionConversionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AttributionConversionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AttributionConversionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AttributionConversionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AttributionConversionResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AttributionConversionResult> for Any {
    fn from(s: AttributionConversionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AttributionConversionResult> for Any {
    fn from(s: &AttributionConversionResult) -> Any {
        s.inner.clone()
    }
}

impl AttributionConversionResult {
    /// Getter of the `report` attribute.
    pub fn report(&self) -> Uint8Array {
        self.inner.get("report").as_::<Uint8Array>()
    }

    /// Setter of the `report` attribute.
    pub fn set_report(&mut self, value: &Uint8Array) {
        self.inner.set("report", value);
    }
}
