use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemFlags {
    inner: Any,
}
impl FromVal for FileSystemFlags {
    fn from_val(v: &Any) -> Self {
        FileSystemFlags { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemFlags {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemFlags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemFlags {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemFlags {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemFlags> for Any {
    fn from(s: FileSystemFlags) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemFlags> for Any {
    fn from(s: &FileSystemFlags) -> Any {
        s.inner.clone()
    }
}

impl FileSystemFlags {
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
impl FileSystemFlags {
    pub fn exclusive(&self) -> bool {
        self.inner.get("exclusive").as_::<bool>()
    }

    pub fn set_exclusive(&mut self, value: bool) {
        self.inner.set("exclusive", value);
    }
}
/// The FileSystemDirectoryEntry class.
/// [`FileSystemDirectoryEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemDirectoryEntry {
    inner: FileSystemEntry,
}
impl FromVal for FileSystemDirectoryEntry {
    fn from_val(v: &Any) -> Self {
        FileSystemDirectoryEntry {
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
impl core::ops::Deref for FileSystemDirectoryEntry {
    type Target = FileSystemEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemDirectoryEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemDirectoryEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemDirectoryEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemDirectoryEntry> for Any {
    fn from(s: FileSystemDirectoryEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemDirectoryEntry> for Any {
    fn from(s: &FileSystemDirectoryEntry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemDirectoryEntry);

impl FileSystemDirectoryEntry {
    /// The createReader method.
    /// [`FileSystemDirectoryEntry.createReader`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/createReader)
    pub fn create_reader(&self) -> FileSystemDirectoryReader {
        self.inner
            .call("createReader", &[])
            .as_::<FileSystemDirectoryReader>()
    }
}
impl FileSystemDirectoryEntry {
    /// The getFile method.
    /// [`FileSystemDirectoryEntry.getFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)
    pub fn get_file0(&self) -> Undefined {
        self.inner.call("getFile", &[]).as_::<Undefined>()
    }
    /// The getFile method.
    /// [`FileSystemDirectoryEntry.getFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)
    pub fn get_file1(&self, path: &USVString) -> Undefined {
        self.inner
            .call("getFile", &[path.into()])
            .as_::<Undefined>()
    }
    /// The getFile method.
    /// [`FileSystemDirectoryEntry.getFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)
    pub fn get_file2(&self, path: &USVString, options: &FileSystemFlags) -> Undefined {
        self.inner
            .call("getFile", &[path.into(), options.into()])
            .as_::<Undefined>()
    }
    /// The getFile method.
    /// [`FileSystemDirectoryEntry.getFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)
    pub fn get_file3(
        &self,
        path: &USVString,
        options: &FileSystemFlags,
        success_callback: &Function,
    ) -> Undefined {
        self.inner
            .call(
                "getFile",
                &[path.into(), options.into(), success_callback.into()],
            )
            .as_::<Undefined>()
    }
    /// The getFile method.
    /// [`FileSystemDirectoryEntry.getFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)
    pub fn get_file4(
        &self,
        path: &USVString,
        options: &FileSystemFlags,
        success_callback: &Function,
        error_callback: &Function,
    ) -> Undefined {
        self.inner
            .call(
                "getFile",
                &[
                    path.into(),
                    options.into(),
                    success_callback.into(),
                    error_callback.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl FileSystemDirectoryEntry {
    /// The getDirectory method.
    /// [`FileSystemDirectoryEntry.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)
    pub fn get_directory0(&self) -> Undefined {
        self.inner.call("getDirectory", &[]).as_::<Undefined>()
    }
    /// The getDirectory method.
    /// [`FileSystemDirectoryEntry.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)
    pub fn get_directory1(&self, path: &USVString) -> Undefined {
        self.inner
            .call("getDirectory", &[path.into()])
            .as_::<Undefined>()
    }
    /// The getDirectory method.
    /// [`FileSystemDirectoryEntry.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)
    pub fn get_directory2(&self, path: &USVString, options: &FileSystemFlags) -> Undefined {
        self.inner
            .call("getDirectory", &[path.into(), options.into()])
            .as_::<Undefined>()
    }
    /// The getDirectory method.
    /// [`FileSystemDirectoryEntry.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)
    pub fn get_directory3(
        &self,
        path: &USVString,
        options: &FileSystemFlags,
        success_callback: &Function,
    ) -> Undefined {
        self.inner
            .call(
                "getDirectory",
                &[path.into(), options.into(), success_callback.into()],
            )
            .as_::<Undefined>()
    }
    /// The getDirectory method.
    /// [`FileSystemDirectoryEntry.getDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)
    pub fn get_directory4(
        &self,
        path: &USVString,
        options: &FileSystemFlags,
        success_callback: &Function,
        error_callback: &Function,
    ) -> Undefined {
        self.inner
            .call(
                "getDirectory",
                &[
                    path.into(),
                    options.into(),
                    success_callback.into(),
                    error_callback.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
