use super::*;

/// The USBPermissionDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBPermissionDescriptor {
    inner: Any,
}

impl FromVal for USBPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        USBPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBPermissionDescriptor> for Any {
    fn from(s: USBPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBPermissionDescriptor> for Any {
    fn from(s: &USBPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl USBPermissionDescriptor {
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
impl USBPermissionDescriptor {
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
