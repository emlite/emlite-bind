use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemHandlePermissionDescriptor {
    inner: emlite::Val,
}
impl FromVal for FileSystemHandlePermissionDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemHandlePermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemHandlePermissionDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemHandlePermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemHandlePermissionDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemHandlePermissionDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemHandlePermissionDescriptor> for emlite::Val {
    fn from(s: FileSystemHandlePermissionDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&FileSystemHandlePermissionDescriptor> for emlite::Val {
    fn from(s: &FileSystemHandlePermissionDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl FileSystemHandlePermissionDescriptor {
    pub fn mode(&self) -> FileSystemPermissionMode {
        self.inner.get("mode").as_::<FileSystemPermissionMode>()
    }

    pub fn set_mode(&mut self, value: FileSystemPermissionMode) {
        self.inner.set("mode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemHandle {
    inner: emlite::Val,
}
impl FromVal for FileSystemHandle {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemHandle {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemHandle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemHandle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemHandle> for emlite::Val {
    fn from(s: FileSystemHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&FileSystemHandle> for emlite::Val {
    fn from(s: &FileSystemHandle) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemHandle);

impl FileSystemHandle {
    pub fn kind(&self) -> FileSystemHandleKind {
        self.inner.get("kind").as_::<FileSystemHandleKind>()
    }
}
impl FileSystemHandle {
    pub fn name(&self) -> USVString {
        self.inner.get("name").as_::<USVString>()
    }
}
impl FileSystemHandle {
    pub fn is_same_entry(&self, other: FileSystemHandle) -> Promise {
        self.inner
            .call("isSameEntry", &[other.into()])
            .as_::<Promise>()
    }
}
impl FileSystemHandle {
    pub fn query_permission0(&self) -> Promise {
        self.inner.call("queryPermission", &[]).as_::<Promise>()
    }

    pub fn query_permission1(&self, descriptor: FileSystemHandlePermissionDescriptor) -> Promise {
        self.inner
            .call("queryPermission", &[descriptor.into()])
            .as_::<Promise>()
    }
}
impl FileSystemHandle {
    pub fn request_permission0(&self) -> Promise {
        self.inner.call("requestPermission", &[]).as_::<Promise>()
    }

    pub fn request_permission1(&self, descriptor: FileSystemHandlePermissionDescriptor) -> Promise {
        self.inner
            .call("requestPermission", &[descriptor.into()])
            .as_::<Promise>()
    }
}
