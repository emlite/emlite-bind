use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for FileSystemWritableFileStream {
    type Target = WritableStream;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileSystemWritableFileStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileSystemWritableFileStream> for emlite::Val {
    fn from(s: FileSystemWritableFileStream) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FileSystemWritableFileStream {
    pub fn write(&self, data: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("write", &[data.into()])
            .as_::<jsbind::Promise>()
    }
}
impl FileSystemWritableFileStream {
    pub fn seek(&self, position: u64) -> jsbind::Promise {
        self.inner
            .call("seek", &[position.into()])
            .as_::<jsbind::Promise>()
    }
}
impl FileSystemWritableFileStream {
    pub fn truncate(&self, size: u64) -> jsbind::Promise {
        self.inner
            .call("truncate", &[size.into()])
            .as_::<jsbind::Promise>()
    }
}
