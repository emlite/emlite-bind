use super::*;

/// The DirectoryPickerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DirectoryPickerOptions {
    inner: Any,
}

impl FromVal for DirectoryPickerOptions {
    fn from_val(v: &Any) -> Self {
        DirectoryPickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DirectoryPickerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DirectoryPickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DirectoryPickerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DirectoryPickerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DirectoryPickerOptions> for Any {
    fn from(s: DirectoryPickerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DirectoryPickerOptions> for Any {
    fn from(s: &DirectoryPickerOptions) -> Any {
        s.inner.clone()
    }
}

impl DirectoryPickerOptions {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl DirectoryPickerOptions {
    /// Getter of the `startIn` attribute.
    pub fn start_in(&self) -> Any {
        self.inner.get("startIn").as_::<Any>()
    }

    /// Setter of the `startIn` attribute.
    pub fn set_start_in(&mut self, value: &Any) {
        self.inner.set("startIn", value);
    }
}
impl DirectoryPickerOptions {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> FileSystemPermissionMode {
        self.inner.get("mode").as_::<FileSystemPermissionMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &FileSystemPermissionMode) {
        self.inner.set("mode", value);
    }
}
