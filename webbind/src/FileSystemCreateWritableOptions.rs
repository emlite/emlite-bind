use super::*;

/// The FileSystemCreateWritableOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemCreateWritableOptions {
    inner: Any,
}

impl FromVal for FileSystemCreateWritableOptions {
    fn from_val(v: &Any) -> Self {
        FileSystemCreateWritableOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemCreateWritableOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemCreateWritableOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemCreateWritableOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemCreateWritableOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FileSystemCreateWritableOptions> for Any {
    fn from(s: FileSystemCreateWritableOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemCreateWritableOptions> for Any {
    fn from(s: &FileSystemCreateWritableOptions) -> Any {
        s.inner.clone()
    }
}

impl FileSystemCreateWritableOptions {
    /// Getter of the `keepExistingData` attribute.
    pub fn keep_existing_data(&self) -> bool {
        self.inner.get("keepExistingData").as_::<bool>()
    }

    /// Setter of the `keepExistingData` attribute.
    pub fn set_keep_existing_data(&mut self, value: bool) {
        self.inner.set("keepExistingData", value);
    }
}
