use super::*;

/// The FileSystemRemoveOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemRemoveOptions {
    inner: Any,
}

impl FromVal for FileSystemRemoveOptions {
    fn from_val(v: &Any) -> Self {
        FileSystemRemoveOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemRemoveOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemRemoveOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemRemoveOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemRemoveOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FileSystemRemoveOptions> for Any {
    fn from(s: FileSystemRemoveOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemRemoveOptions> for Any {
    fn from(s: &FileSystemRemoveOptions) -> Any {
        s.inner.clone()
    }
}

impl FileSystemRemoveOptions {
    /// Getter of the `recursive` attribute.
    pub fn recursive(&self) -> bool {
        self.inner.get("recursive").as_::<bool>()
    }

    /// Setter of the `recursive` attribute.
    pub fn set_recursive(&mut self, value: bool) {
        self.inner.set("recursive", value);
    }
}
