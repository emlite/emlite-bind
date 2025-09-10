use super::*;

/// The FileSystemFlags dictionary.
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
    /// Getter of the `create` attribute.
    pub fn create(&self) -> bool {
        self.inner.get("create").as_::<bool>()
    }

    /// Setter of the `create` attribute.
    pub fn set_create(&mut self, value: bool) {
        self.inner.set("create", value);
    }
}
impl FileSystemFlags {
    /// Getter of the `exclusive` attribute.
    pub fn exclusive(&self) -> bool {
        self.inner.get("exclusive").as_::<bool>()
    }

    /// Setter of the `exclusive` attribute.
    pub fn set_exclusive(&mut self, value: bool) {
        self.inner.set("exclusive", value);
    }
}
