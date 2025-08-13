use super::*;




/// The FileSystemHandlePermissionDescriptor dictionary.
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
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> FileSystemPermissionMode {
        self.inner.get("mode").as_::<FileSystemPermissionMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &FileSystemPermissionMode) {
        self.inner.set("mode", value);
    }
}
