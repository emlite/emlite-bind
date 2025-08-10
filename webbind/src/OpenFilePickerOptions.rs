use super::*;

/// The OpenFilePickerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OpenFilePickerOptions {
    inner: Any,
}

impl FromVal for OpenFilePickerOptions {
    fn from_val(v: &Any) -> Self {
        OpenFilePickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OpenFilePickerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OpenFilePickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OpenFilePickerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OpenFilePickerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<OpenFilePickerOptions> for Any {
    fn from(s: OpenFilePickerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OpenFilePickerOptions> for Any {
    fn from(s: &OpenFilePickerOptions) -> Any {
        s.inner.clone()
    }
}

impl OpenFilePickerOptions {
    /// Getter of the `multiple` attribute.
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    /// Setter of the `multiple` attribute.
    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
