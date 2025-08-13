use super::*;




/// The FileSystemFileHandle class.
/// [`FileSystemFileHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemFileHandle {
    inner: FileSystemHandle,
}

impl FromVal for FileSystemFileHandle {
    fn from_val(v: &Any) -> Self {
        FileSystemFileHandle { inner: FileSystemHandle::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemFileHandle {
    type Target = FileSystemHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemFileHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemFileHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemFileHandle {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FileSystemFileHandle> for Any {
    fn from(s: FileSystemFileHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemFileHandle> for Any {
    fn from(s: &FileSystemFileHandle) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FileSystemFileHandle);


impl FileSystemFileHandle {
    /// The getFile method.
    /// [`FileSystemFileHandle.getFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile)
    pub fn get_file(&self, ) -> Promise<File> {
        self.inner.call("getFile", &[]).as_::<Promise<File>>()
    }
}
impl FileSystemFileHandle {
    /// The createWritable method.
    /// [`FileSystemFileHandle.createWritable`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)
    pub fn create_writable0(&self, ) -> Promise<FileSystemWritableFileStream> {
        self.inner.call("createWritable", &[]).as_::<Promise<FileSystemWritableFileStream>>()
    }
    /// The createWritable method.
    /// [`FileSystemFileHandle.createWritable`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)
    pub fn create_writable1(&self, options: &FileSystemCreateWritableOptions) -> Promise<FileSystemWritableFileStream> {
        self.inner.call("createWritable", &[options.into(), ]).as_::<Promise<FileSystemWritableFileStream>>()
    }
}
impl FileSystemFileHandle {
    /// The createSyncAccessHandle method.
    /// [`FileSystemFileHandle.createSyncAccessHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createSyncAccessHandle)
    pub fn create_sync_access_handle(&self, ) -> Promise<FileSystemSyncAccessHandle> {
        self.inner.call("createSyncAccessHandle", &[]).as_::<Promise<FileSystemSyncAccessHandle>>()
    }
}
