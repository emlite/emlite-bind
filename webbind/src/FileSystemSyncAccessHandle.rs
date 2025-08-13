use super::*;




/// The FileSystemSyncAccessHandle class.
/// [`FileSystemSyncAccessHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemSyncAccessHandle {
    inner: Any,
}

impl FromVal for FileSystemSyncAccessHandle {
    fn from_val(v: &Any) -> Self {
        FileSystemSyncAccessHandle { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemSyncAccessHandle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemSyncAccessHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemSyncAccessHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemSyncAccessHandle {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FileSystemSyncAccessHandle> for Any {
    fn from(s: FileSystemSyncAccessHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemSyncAccessHandle> for Any {
    fn from(s: &FileSystemSyncAccessHandle) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FileSystemSyncAccessHandle);


impl FileSystemSyncAccessHandle {
    /// The read method.
    /// [`FileSystemSyncAccessHandle.read`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)
    pub fn read0(&self, buffer: &Any) -> u64 {
        self.inner.call("read", &[buffer.into(), ]).as_::<u64>()
    }
    /// The read method.
    /// [`FileSystemSyncAccessHandle.read`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)
    pub fn read1(&self, buffer: &Any, options: &FileSystemReadWriteOptions) -> u64 {
        self.inner.call("read", &[buffer.into(), options.into(), ]).as_::<u64>()
    }
}
impl FileSystemSyncAccessHandle {
    /// The write method.
    /// [`FileSystemSyncAccessHandle.write`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)
    pub fn write0(&self, buffer: &Any) -> u64 {
        self.inner.call("write", &[buffer.into(), ]).as_::<u64>()
    }
    /// The write method.
    /// [`FileSystemSyncAccessHandle.write`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)
    pub fn write1(&self, buffer: &Any, options: &FileSystemReadWriteOptions) -> u64 {
        self.inner.call("write", &[buffer.into(), options.into(), ]).as_::<u64>()
    }
}
impl FileSystemSyncAccessHandle {
    /// The truncate method.
    /// [`FileSystemSyncAccessHandle.truncate`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/truncate)
    pub fn truncate(&self, new_size: u64) -> Undefined {
        self.inner.call("truncate", &[new_size.into(), ]).as_::<Undefined>()
    }
}
impl FileSystemSyncAccessHandle {
    /// The getSize method.
    /// [`FileSystemSyncAccessHandle.getSize`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/getSize)
    pub fn get_size(&self, ) -> u64 {
        self.inner.call("getSize", &[]).as_::<u64>()
    }
}
impl FileSystemSyncAccessHandle {
    /// The flush method.
    /// [`FileSystemSyncAccessHandle.flush`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/flush)
    pub fn flush(&self, ) -> Undefined {
        self.inner.call("flush", &[]).as_::<Undefined>()
    }
}
impl FileSystemSyncAccessHandle {
    /// The close method.
    /// [`FileSystemSyncAccessHandle.close`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
