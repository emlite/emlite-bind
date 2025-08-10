use super::*;

/// The FileSystemReadWriteOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemReadWriteOptions {
    inner: Any,
}

impl FromVal for FileSystemReadWriteOptions {
    fn from_val(v: &Any) -> Self {
        FileSystemReadWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemReadWriteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemReadWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemReadWriteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemReadWriteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FileSystemReadWriteOptions> for Any {
    fn from(s: FileSystemReadWriteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemReadWriteOptions> for Any {
    fn from(s: &FileSystemReadWriteOptions) -> Any {
        s.inner.clone()
    }
}

impl FileSystemReadWriteOptions {
    /// Getter of the `at` attribute.
    pub fn at(&self) -> u64 {
        self.inner.get("at").as_::<u64>()
    }

    /// Setter of the `at` attribute.
    pub fn set_at(&mut self, value: u64) {
        self.inner.set("at", value);
    }
}
