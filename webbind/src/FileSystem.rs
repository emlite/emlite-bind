use super::*;

/// The FileSystem class.
/// [`FileSystem`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystem {
    inner: Any,
}
impl FromVal for FileSystem {
    fn from_val(v: &Any) -> Self {
        FileSystem {
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
impl core::ops::Deref for FileSystem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileSystem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileSystem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileSystem> for Any {
    fn from(s: FileSystem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileSystem> for Any {
    fn from(s: &FileSystem) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileSystem);

impl FileSystem {
    /// Getter of the `name` attribute.
    /// [`FileSystem.name`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl FileSystem {
    /// Getter of the `root` attribute.
    /// [`FileSystem.root`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/root)
    pub fn root(&self) -> FileSystemDirectoryEntry {
        self.inner.get("root").as_::<FileSystemDirectoryEntry>()
    }
}
