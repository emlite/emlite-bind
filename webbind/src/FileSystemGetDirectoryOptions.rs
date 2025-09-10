use super::*;

/// The FileSystemGetDirectoryOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemGetDirectoryOptions {
    inner: Any,
}

impl FromVal for FileSystemGetDirectoryOptions {
    fn from_val(v: &Any) -> Self {
        FileSystemGetDirectoryOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemGetDirectoryOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemGetDirectoryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemGetDirectoryOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemGetDirectoryOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FileSystemGetDirectoryOptions> for Any {
    fn from(s: FileSystemGetDirectoryOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemGetDirectoryOptions> for Any {
    fn from(s: &FileSystemGetDirectoryOptions) -> Any {
        s.inner.clone()
    }
}

impl FileSystemGetDirectoryOptions {
    /// Getter of the `create` attribute.
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    /// Setter of the `create` attribute.
    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
