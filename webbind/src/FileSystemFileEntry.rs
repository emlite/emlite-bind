use super::*;

/// The FileSystemFileEntry class.
/// [`FileSystemFileEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemFileEntry {
    inner: FileSystemEntry,
}
impl FromVal for FileSystemFileEntry {
    fn from_val(v: &Any) -> Self {
        FileSystemFileEntry {
            inner: FileSystemEntry::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemFileEntry {
    type Target = FileSystemEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemFileEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemFileEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemFileEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemFileEntry> for Any {
    fn from(s: FileSystemFileEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemFileEntry> for Any {
    fn from(s: &FileSystemFileEntry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemFileEntry);

impl FileSystemFileEntry {
    /// The file method.
    /// [`FileSystemFileEntry.file`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)
    pub fn file0(&self, success_callback: &Function) -> Undefined {
        self.inner
            .call("file", &[success_callback.into()])
            .as_::<Undefined>()
    }
    /// The file method.
    /// [`FileSystemFileEntry.file`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)
    pub fn file1(&self, success_callback: &Function, error_callback: &Function) -> Undefined {
        self.inner
            .call("file", &[success_callback.into(), error_callback.into()])
            .as_::<Undefined>()
    }
}
