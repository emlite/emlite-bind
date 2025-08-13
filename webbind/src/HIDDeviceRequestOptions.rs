use super::*;




/// The HIDDeviceRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDDeviceRequestOptions {
    inner: Any,
}

impl FromVal for HIDDeviceRequestOptions {
    fn from_val(v: &Any) -> Self {
        HIDDeviceRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HIDDeviceRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HIDDeviceRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HIDDeviceRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HIDDeviceRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HIDDeviceRequestOptions> for Any {
    fn from(s: HIDDeviceRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HIDDeviceRequestOptions> for Any {
    fn from(s: &HIDDeviceRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl HIDDeviceRequestOptions {
    /// Getter of the `filters` attribute.
    pub fn filters(&self) -> TypedArray<HIDDeviceFilter> {
        self.inner.get("filters").as_::<TypedArray<HIDDeviceFilter>>()
    }

    /// Setter of the `filters` attribute.
    pub fn set_filters(&mut self, value: &TypedArray<HIDDeviceFilter>) {
        self.inner.set("filters", value);
    }
}
impl HIDDeviceRequestOptions {
    /// Getter of the `exclusionFilters` attribute.
    pub fn exclusion_filters(&self) -> TypedArray<HIDDeviceFilter> {
        self.inner.get("exclusionFilters").as_::<TypedArray<HIDDeviceFilter>>()
    }

    /// Setter of the `exclusionFilters` attribute.
    pub fn set_exclusion_filters(&mut self, value: &TypedArray<HIDDeviceFilter>) {
        self.inner.set("exclusionFilters", value);
    }
}
