use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for FileSystemHandlePermissionDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemHandlePermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemHandlePermissionDescriptor> for emlite::Val {
    fn from(s: FileSystemHandlePermissionDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for FileSystemHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemHandle> for emlite::Val {
    fn from(s: FileSystemHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemHandle {
    pub fn kind(&self) -> FileSystemHandleKind {
        self.inner.get("kind").as_::<FileSystemHandleKind>()
    }
}
impl FileSystemHandle {
    pub fn name(&self) -> jsbind::USVString {
        self.inner.get("name").as_::<jsbind::USVString>()
    }
}
impl FileSystemHandle {
    pub fn is_same_entry(&self, other: FileSystemHandle) -> jsbind::Promise {
        self.inner
            .call("isSameEntry", &[other.into()])
            .as_::<jsbind::Promise>()
    }
}
impl FileSystemHandle {
    pub fn query_permission0(&self) -> jsbind::Promise {
        self.inner
            .call("queryPermission", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn query_permission1(
        &self,
        descriptor: FileSystemHandlePermissionDescriptor,
    ) -> jsbind::Promise {
        self.inner
            .call("queryPermission", &[descriptor.into()])
            .as_::<jsbind::Promise>()
    }
}
impl FileSystemHandle {
    pub fn request_permission0(&self) -> jsbind::Promise {
        self.inner
            .call("requestPermission", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_permission1(
        &self,
        descriptor: FileSystemHandlePermissionDescriptor,
    ) -> jsbind::Promise {
        self.inner
            .call("requestPermission", &[descriptor.into()])
            .as_::<jsbind::Promise>()
    }
}
