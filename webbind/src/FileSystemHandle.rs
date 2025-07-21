use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemHandlePermissionDescriptor {
    inner: Any,
}
impl FromVal for FileSystemHandlePermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        FileSystemHandlePermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemHandlePermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemHandlePermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemHandlePermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemHandlePermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemHandlePermissionDescriptor> for Any {
    fn from(s: FileSystemHandlePermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemHandlePermissionDescriptor> for Any {
    fn from(s: &FileSystemHandlePermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl FileSystemHandlePermissionDescriptor {
    pub fn mode(&self) -> FileSystemPermissionMode {
        self.inner.get("mode").as_::<FileSystemPermissionMode>()
    }

    pub fn set_mode(&mut self, value: &FileSystemPermissionMode) {
        self.inner.set("mode", value);
    }
}
/// The FileSystemHandle class.
/// [`FileSystemHandle`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemHandle {
    inner: Any,
}
impl FromVal for FileSystemHandle {
    fn from_val(v: &Any) -> Self {
        FileSystemHandle {
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
impl core::ops::Deref for FileSystemHandle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemHandle {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemHandle> for Any {
    fn from(s: FileSystemHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemHandle> for Any {
    fn from(s: &FileSystemHandle) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemHandle);

impl FileSystemHandle {
    /// Getter of the `kind` attribute.
    /// [`FileSystemHandle.kind`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/kind)
    pub fn kind(&self) -> FileSystemHandleKind {
        self.inner.get("kind").as_::<FileSystemHandleKind>()
    }
}
impl FileSystemHandle {
    /// Getter of the `name` attribute.
    /// [`FileSystemHandle.name`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl FileSystemHandle {
    /// The isSameEntry method.
    /// [`FileSystemHandle.isSameEntry`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/isSameEntry)
    pub fn is_same_entry(&self, other: &FileSystemHandle) -> Promise<bool> {
        self.inner
            .call("isSameEntry", &[other.into()])
            .as_::<Promise<bool>>()
    }
}
impl FileSystemHandle {
    /// The queryPermission method.
    /// [`FileSystemHandle.queryPermission`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)
    pub fn query_permission0(&self) -> Promise<PermissionState> {
        self.inner
            .call("queryPermission", &[])
            .as_::<Promise<PermissionState>>()
    }
    /// The queryPermission method.
    /// [`FileSystemHandle.queryPermission`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)
    pub fn query_permission1(
        &self,
        descriptor: &FileSystemHandlePermissionDescriptor,
    ) -> Promise<PermissionState> {
        self.inner
            .call("queryPermission", &[descriptor.into()])
            .as_::<Promise<PermissionState>>()
    }
}
impl FileSystemHandle {
    /// The requestPermission method.
    /// [`FileSystemHandle.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)
    pub fn request_permission0(&self) -> Promise<PermissionState> {
        self.inner
            .call("requestPermission", &[])
            .as_::<Promise<PermissionState>>()
    }
    /// The requestPermission method.
    /// [`FileSystemHandle.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)
    pub fn request_permission1(
        &self,
        descriptor: &FileSystemHandlePermissionDescriptor,
    ) -> Promise<PermissionState> {
        self.inner
            .call("requestPermission", &[descriptor.into()])
            .as_::<Promise<PermissionState>>()
    }
}
