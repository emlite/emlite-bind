use super::*;

/// The USBDeviceRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBDeviceRequestOptions {
    inner: Any,
}

impl FromVal for USBDeviceRequestOptions {
    fn from_val(v: &Any) -> Self {
        USBDeviceRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBDeviceRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBDeviceRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBDeviceRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBDeviceRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBDeviceRequestOptions> for Any {
    fn from(s: USBDeviceRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBDeviceRequestOptions> for Any {
    fn from(s: &USBDeviceRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl USBDeviceRequestOptions {
    /// Getter of the `filters` attribute.
    pub fn filters(&self) -> TypedArray<USBDeviceFilter> {
        self.inner
            .get("filters")
            .as_::<TypedArray<USBDeviceFilter>>()
    }

    /// Setter of the `filters` attribute.
    pub fn set_filters(&mut self, value: &TypedArray<USBDeviceFilter>) {
        self.inner.set("filters", value);
    }
}
impl USBDeviceRequestOptions {
    /// Getter of the `exclusionFilters` attribute.
    pub fn exclusion_filters(&self) -> TypedArray<USBDeviceFilter> {
        self.inner
            .get("exclusionFilters")
            .as_::<TypedArray<USBDeviceFilter>>()
    }

    /// Setter of the `exclusionFilters` attribute.
    pub fn set_exclusion_filters(&mut self, value: &TypedArray<USBDeviceFilter>) {
        self.inner.set("exclusionFilters", value);
    }
}
