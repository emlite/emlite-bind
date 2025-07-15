use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemWritableFileStream {
    inner: WritableStream,
}
impl FromVal for FileSystemWritableFileStream {
    fn from_val(v: &emlite::Val) -> Self {
        FileSystemWritableFileStream {
            inner: WritableStream::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FileSystemWritableFileStream {
    type Target = WritableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileSystemWritableFileStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileSystemWritableFileStream {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileSystemWritableFileStream {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileSystemWritableFileStream> for emlite::Val {
    fn from(s: FileSystemWritableFileStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FileSystemWritableFileStream);

impl FileSystemWritableFileStream {
    pub fn write(&self, data: Any) -> Promise {
        self.inner.call("write", &[data.into()]).as_::<Promise>()
    }
}
impl FileSystemWritableFileStream {
    pub fn seek(&self, position: u64) -> Promise {
        self.inner.call("seek", &[position.into()]).as_::<Promise>()
    }
}
impl FileSystemWritableFileStream {
    pub fn truncate(&self, size: u64) -> Promise {
        self.inner.call("truncate", &[size.into()]).as_::<Promise>()
    }
}
