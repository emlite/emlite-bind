use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileReaderSync {
    inner: emlite::Val,
}
impl FromVal for FileReaderSync {
    fn from_val(v: &emlite::Val) -> Self {
        FileReaderSync {
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
impl core::ops::Deref for FileReaderSync {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FileReaderSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FileReaderSync {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FileReaderSync {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FileReaderSync> for emlite::Val {
    fn from(s: FileReaderSync) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FileReaderSync);

impl FileReaderSync {
    pub fn new() -> FileReaderSync {
        Self {
            inner: emlite::Val::global("FileReaderSync")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl FileReaderSync {
    pub fn read_as_array_buffer(&self, blob: Blob) -> ArrayBuffer {
        self.inner
            .call("readAsArrayBuffer", &[blob.into()])
            .as_::<ArrayBuffer>()
    }
}
impl FileReaderSync {
    pub fn read_as_binary_string(&self, blob: Blob) -> DOMString {
        self.inner
            .call("readAsBinaryString", &[blob.into()])
            .as_::<DOMString>()
    }
}
impl FileReaderSync {
    pub fn read_as_text0(&self, blob: Blob) -> DOMString {
        self.inner
            .call("readAsText", &[blob.into()])
            .as_::<DOMString>()
    }

    pub fn read_as_text1(&self, blob: Blob, encoding: DOMString) -> DOMString {
        self.inner
            .call("readAsText", &[blob.into(), encoding.into()])
            .as_::<DOMString>()
    }
}
impl FileReaderSync {
    pub fn read_as_data_url(&self, blob: Blob) -> DOMString {
        self.inner
            .call("readAsDataURL", &[blob.into()])
            .as_::<DOMString>()
    }
}
