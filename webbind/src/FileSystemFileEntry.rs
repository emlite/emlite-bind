use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemFileEntry {
    inner: FileSystemEntry,
}
impl FromVal for FileSystemFileEntry {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemFileEntry { inner: FileSystemEntry::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemFileEntry {
    type Target = FileSystemEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemFileEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemFileEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemFileEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<FileSystemFileEntry> for emlite::Val {
    fn from(s: FileSystemFileEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemFileEntry);


impl FileSystemFileEntry {
    pub fn file0(&self, success_callback: Function) -> Undefined {
        self.inner.call("file", &[success_callback.into(), ]).as_::<Undefined>()
    }

    pub fn file1(&self, success_callback: Function, error_callback: Function) -> Undefined {
        self.inner.call("file", &[success_callback.into(), error_callback.into(), ]).as_::<Undefined>()
    }

}
