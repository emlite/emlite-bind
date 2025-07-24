use super::*;

/// The FileSystemEntry class.
/// [`FileSystemEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemEntry {
    inner: Any,
}
impl FromVal for FileSystemEntry {
    fn from_val(v: &Any) -> Self {
        FileSystemEntry {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemEntry> for Any {
    fn from(s: FileSystemEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemEntry> for Any {
    fn from(s: &FileSystemEntry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemEntry);

impl FileSystemEntry {
    /// Getter of the `isFile` attribute.
    /// [`FileSystemEntry.isFile`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isFile)
    pub fn is_file(&self) -> bool {
        self.inner.get("isFile").as_::<bool>()
    }
}
impl FileSystemEntry {
    /// Getter of the `isDirectory` attribute.
    /// [`FileSystemEntry.isDirectory`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isDirectory)
    pub fn is_directory(&self) -> bool {
        self.inner.get("isDirectory").as_::<bool>()
    }
}
impl FileSystemEntry {
    /// Getter of the `name` attribute.
    /// [`FileSystemEntry.name`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/name)
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }
}
impl FileSystemEntry {
    /// Getter of the `fullPath` attribute.
    /// [`FileSystemEntry.fullPath`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/fullPath)
    pub fn full_path(&self) -> USVString {
        self.inner.get("fullPath").as_::<USVString>()
    }
}
impl FileSystemEntry {
    /// Getter of the `filesystem` attribute.
    /// [`FileSystemEntry.filesystem`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/filesystem)
    pub fn filesystem(&self) -> FileSystem {
        self.inner.get("filesystem").as_::<FileSystem>()
    }
}
impl FileSystemEntry {
    /// The getParent method.
    /// [`FileSystemEntry.getParent`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    pub fn get_parent0(&self) -> Undefined {
        self.inner.call("getParent", &[]).as_::<Undefined>()
    }
    /// The getParent method.
    /// [`FileSystemEntry.getParent`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    pub fn get_parent1(&self, success_callback: &Function) -> Undefined {
        self.inner
            .call("getParent", &[success_callback.into()])
            .as_::<Undefined>()
    }
    /// The getParent method.
    /// [`FileSystemEntry.getParent`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)
    pub fn get_parent2(&self, success_callback: &Function, error_callback: &Function) -> Undefined {
        self.inner
            .call(
                "getParent",
                &[success_callback.into(), error_callback.into()],
            )
            .as_::<Undefined>()
    }
}
