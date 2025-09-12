use super::*;

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
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
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
    pub fn query_permission(&self) -> Promise<PermissionState> {
        self.inner
            .call("queryPermission", &[])
            .as_::<Promise<PermissionState>>()
    }
}
impl FileSystemHandle {
    /// The queryPermission method.
    /// [`FileSystemHandle.queryPermission`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)
    pub fn query_permission_with_descriptor(
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
    pub fn request_permission(&self) -> Promise<PermissionState> {
        self.inner
            .call("requestPermission", &[])
            .as_::<Promise<PermissionState>>()
    }
}
impl FileSystemHandle {
    /// The requestPermission method.
    /// [`FileSystemHandle.requestPermission`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)
    pub fn request_permission_with_descriptor(
        &self,
        descriptor: &FileSystemHandlePermissionDescriptor,
    ) -> Promise<PermissionState> {
        self.inner
            .call("requestPermission", &[descriptor.into()])
            .as_::<Promise<PermissionState>>()
    }
}
