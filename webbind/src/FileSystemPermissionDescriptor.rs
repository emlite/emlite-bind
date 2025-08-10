use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemPermissionDescriptor {
    inner: Any,
}
impl FromVal for FileSystemPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        FileSystemPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystemPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystemPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystemPermissionDescriptor> for Any {
    fn from(s: FileSystemPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystemPermissionDescriptor> for Any {
    fn from(s: &FileSystemPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl FileSystemPermissionDescriptor {
    pub fn handle(&self) -> FileSystemHandle {
        self.inner.get("handle").as_::<FileSystemHandle>()
    }

    pub fn set_handle(&mut self, value: &FileSystemHandle) {
        self.inner.set("handle", value);
    }
}
impl FileSystemPermissionDescriptor {
    pub fn mode(&self) -> FileSystemPermissionMode {
        self.inner.get("mode").as_::<FileSystemPermissionMode>()
    }

    pub fn set_mode(&mut self, value: &FileSystemPermissionMode) {
        self.inner.set("mode", value);
    }
}
