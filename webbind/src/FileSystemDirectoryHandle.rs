use super::*;




/// The FileSystemDirectoryHandle class.
/// [`FileSystemDirectoryHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemDirectoryHandle {
    inner: FileSystemHandle,
}

impl FromVal for FileSystemDirectoryHandle {
    fn from_val(v: &Any) -> Self {
        FileSystemDirectoryHandle { inner: FileSystemHandle::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FileSystemDirectoryHandle {
    type Target = FileSystemHandle;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileSystemDirectoryHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileSystemDirectoryHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemDirectoryHandle {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FileSystemDirectoryHandle> for Any {
    fn from(s: FileSystemDirectoryHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemDirectoryHandle> for Any {
    fn from(s: &FileSystemDirectoryHandle) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FileSystemDirectoryHandle);


impl FileSystemDirectoryHandle {
    /// The getFileHandle method.
    /// [`FileSystemDirectoryHandle.getFileHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)
    pub fn get_file_handle0(&self, name: &JsString) -> Promise<FileSystemFileHandle> {
        self.inner.call("getFileHandle", &[name.into(), ]).as_::<Promise<FileSystemFileHandle>>()
    }
    /// The getFileHandle method.
    /// [`FileSystemDirectoryHandle.getFileHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)
    pub fn get_file_handle1(&self, name: &JsString, options: &FileSystemGetFileOptions) -> Promise<FileSystemFileHandle> {
        self.inner.call("getFileHandle", &[name.into(), options.into(), ]).as_::<Promise<FileSystemFileHandle>>()
    }
}
impl FileSystemDirectoryHandle {
    /// The getDirectoryHandle method.
    /// [`FileSystemDirectoryHandle.getDirectoryHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)
    pub fn get_directory_handle0(&self, name: &JsString) -> Promise<FileSystemDirectoryHandle> {
        self.inner.call("getDirectoryHandle", &[name.into(), ]).as_::<Promise<FileSystemDirectoryHandle>>()
    }
    /// The getDirectoryHandle method.
    /// [`FileSystemDirectoryHandle.getDirectoryHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)
    pub fn get_directory_handle1(&self, name: &JsString, options: &FileSystemGetDirectoryOptions) -> Promise<FileSystemDirectoryHandle> {
        self.inner.call("getDirectoryHandle", &[name.into(), options.into(), ]).as_::<Promise<FileSystemDirectoryHandle>>()
    }
}
impl FileSystemDirectoryHandle {
    /// The removeEntry method.
    /// [`FileSystemDirectoryHandle.removeEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)
    pub fn remove_entry0(&self, name: &JsString) -> Promise<Undefined> {
        self.inner.call("removeEntry", &[name.into(), ]).as_::<Promise<Undefined>>()
    }
    /// The removeEntry method.
    /// [`FileSystemDirectoryHandle.removeEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)
    pub fn remove_entry1(&self, name: &JsString, options: &FileSystemRemoveOptions) -> Promise<Undefined> {
        self.inner.call("removeEntry", &[name.into(), options.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl FileSystemDirectoryHandle {
    /// The resolve method.
    /// [`FileSystemDirectoryHandle.resolve`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve)
    pub fn resolve(&self, possible_descendant: &FileSystemHandle) -> Promise<TypedArray<JsString>> {
        self.inner.call("resolve", &[possible_descendant.into(), ]).as_::<Promise<TypedArray<JsString>>>()
    }
}
