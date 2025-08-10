use super::*;

/// The FileSystemWritableFileStream class.
/// [`FileSystemWritableFileStream`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileSystemWritableFileStream {
    inner: WritableStream,
}

impl FromVal for FileSystemWritableFileStream {
    fn from_val(v: &Any) -> Self {
        FileSystemWritableFileStream {
            inner: WritableStream::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for FileSystemWritableFileStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileSystemWritableFileStream {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FileSystemWritableFileStream> for Any {
    fn from(s: FileSystemWritableFileStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileSystemWritableFileStream> for Any {
    fn from(s: &FileSystemWritableFileStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FileSystemWritableFileStream);

impl FileSystemWritableFileStream {
    /// The write method.
    /// [`FileSystemWritableFileStream.write`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)
    pub fn write(&self, data: &Any) -> Promise<Undefined> {
        self.inner
            .call("write", &[data.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl FileSystemWritableFileStream {
    /// The seek method.
    /// [`FileSystemWritableFileStream.seek`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)
    pub fn seek(&self, position: u64) -> Promise<Undefined> {
        self.inner
            .call("seek", &[position.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl FileSystemWritableFileStream {
    /// The truncate method.
    /// [`FileSystemWritableFileStream.truncate`](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)
    pub fn truncate(&self, size: u64) -> Promise<Undefined> {
        self.inner
            .call("truncate", &[size.into()])
            .as_::<Promise<Undefined>>()
    }
}
