use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemGetFileOptions {
    inner: Any,
}
impl FromVal for FileSystemGetFileOptions {
    fn from_val(v: &Any) -> Self {
        FileSystemGetFileOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemGetFileOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemGetFileOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemGetFileOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemGetFileOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemGetFileOptions> for Any {
    fn from(s: FileSystemGetFileOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemGetFileOptions> for Any {
    fn from(s: &FileSystemGetFileOptions) -> Any {
        s.inner.clone()
    }
}

impl FileSystemGetFileOptions {
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
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
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
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
    pub fn recursive(&self) -> bool {
        self.inner.get("recursive").as_::<bool>()
    }

    pub fn set_recursive(&mut self, value: bool) {
        self.inner.set("recursive", value);
    }
}
/// The FileSystemDirectoryHandle class.
/// [`FileSystemDirectoryHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemDirectoryHandle {
    inner: FileSystemHandle,
}
impl FromVal for FileSystemDirectoryHandle {
    fn from_val(v: &Any) -> Self {
        FileSystemDirectoryHandle {
            inner: FileSystemHandle::from_val(v),
        }
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
    pub fn get_file_handle0(&self, name: &USVString) -> Promise<FileSystemFileHandle> {
        self.inner
            .call("getFileHandle", &[name.into()])
            .as_::<Promise<FileSystemFileHandle>>()
    }
    /// The getFileHandle method.
    /// [`FileSystemDirectoryHandle.getFileHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)
    pub fn get_file_handle1(
        &self,
        name: &USVString,
        options: &FileSystemGetFileOptions,
    ) -> Promise<FileSystemFileHandle> {
        self.inner
            .call("getFileHandle", &[name.into(), options.into()])
            .as_::<Promise<FileSystemFileHandle>>()
    }
}
impl FileSystemDirectoryHandle {
    /// The getDirectoryHandle method.
    /// [`FileSystemDirectoryHandle.getDirectoryHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)
    pub fn get_directory_handle0(&self, name: &USVString) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("getDirectoryHandle", &[name.into()])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
    /// The getDirectoryHandle method.
    /// [`FileSystemDirectoryHandle.getDirectoryHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)
    pub fn get_directory_handle1(
        &self,
        name: &USVString,
        options: &FileSystemGetDirectoryOptions,
    ) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("getDirectoryHandle", &[name.into(), options.into()])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
}
impl FileSystemDirectoryHandle {
    /// The removeEntry method.
    /// [`FileSystemDirectoryHandle.removeEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)
    pub fn remove_entry0(&self, name: &USVString) -> Promise<Undefined> {
        self.inner
            .call("removeEntry", &[name.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The removeEntry method.
    /// [`FileSystemDirectoryHandle.removeEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)
    pub fn remove_entry1(
        &self,
        name: &USVString,
        options: &FileSystemRemoveOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call("removeEntry", &[name.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl FileSystemDirectoryHandle {
    /// The resolve method.
    /// [`FileSystemDirectoryHandle.resolve`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve)
    pub fn resolve(&self, possible_descendant: &FileSystemHandle) -> Promise<Sequence<USVString>> {
        self.inner
            .call("resolve", &[possible_descendant.into()])
            .as_::<Promise<Sequence<USVString>>>()
    }
}
