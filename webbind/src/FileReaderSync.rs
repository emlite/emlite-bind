use super::*;

/// The FileReaderSync class.
/// [`FileReaderSync`](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileReaderSync {
    inner: Any,
}
impl FromVal for FileReaderSync {
    fn from_val(v: &Any) -> Self {
        FileReaderSync {
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
impl core::ops::Deref for FileReaderSync {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileReaderSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FileReaderSync {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FileReaderSync {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FileReaderSync> for Any {
    fn from(s: FileReaderSync) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FileReaderSync> for Any {
    fn from(s: &FileReaderSync) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FileReaderSync);

impl FileReaderSync {
    /// The `new FileReaderSync(..)` constructor, creating a new FileReaderSync instance
    pub fn new() -> FileReaderSync {
        Self {
            inner: Any::global("FileReaderSync").new(&[]).as_::<Any>(),
        }
    }
}
impl FileReaderSync {
    /// The readAsArrayBuffer method.
    /// [`FileReaderSync.readAsArrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsArrayBuffer)
    pub fn read_as_array_buffer(&self, blob: &Blob) -> ArrayBuffer {
        self.inner
            .call("readAsArrayBuffer", &[blob.into()])
            .as_::<ArrayBuffer>()
    }
}
impl FileReaderSync {
    /// The readAsBinaryString method.
    /// [`FileReaderSync.readAsBinaryString`](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsBinaryString)
    pub fn read_as_binary_string(&self, blob: &Blob) -> String {
        self.inner
            .call("readAsBinaryString", &[blob.into()])
            .as_::<String>()
    }
}
impl FileReaderSync {
    /// The readAsText method.
    /// [`FileReaderSync.readAsText`](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)
    pub fn read_as_text0(&self, blob: &Blob) -> String {
        self.inner
            .call("readAsText", &[blob.into()])
            .as_::<String>()
    }
    /// The readAsText method.
    /// [`FileReaderSync.readAsText`](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)
    pub fn read_as_text1(&self, blob: &Blob, encoding: &str) -> String {
        self.inner
            .call("readAsText", &[blob.into(), encoding.into()])
            .as_::<String>()
    }
}
impl FileReaderSync {
    /// The readAsDataURL method.
    /// [`FileReaderSync.readAsDataURL`](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsDataURL)
    pub fn read_as_data_url(&self, blob: &Blob) -> String {
        self.inner
            .call("readAsDataURL", &[blob.into()])
            .as_::<String>()
    }
}
