use super::*;




/// The FileSystemDirectoryReader class.
/// [`FileSystemDirectoryReader`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemDirectoryReader {
    inner: Any,
}

impl FromVal for FileSystemDirectoryReader {
    fn from_val(v: &Any) -> Self {
        FileSystemDirectoryReader { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemDirectoryReader {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemDirectoryReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemDirectoryReader {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemDirectoryReader {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FileSystemDirectoryReader> for Any {
    fn from(s: FileSystemDirectoryReader) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemDirectoryReader> for Any {
    fn from(s: &FileSystemDirectoryReader) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FileSystemDirectoryReader);


impl FileSystemDirectoryReader {
    /// The readEntries method.
    /// [`FileSystemDirectoryReader.readEntries`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)
    pub fn read_entries0(&self, success_callback: &Function) -> Undefined {
        self.inner.call("readEntries", &[success_callback.into(), ]).as_::<Undefined>()
    }
    /// The readEntries method.
    /// [`FileSystemDirectoryReader.readEntries`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)
    pub fn read_entries1(&self, success_callback: &Function, error_callback: &Function) -> Undefined {
        self.inner.call("readEntries", &[success_callback.into(), error_callback.into(), ]).as_::<Undefined>()
    }
}
