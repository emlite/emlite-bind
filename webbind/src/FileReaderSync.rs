use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for FileReaderSync {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FileReaderSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FileReaderSync> for emlite::Val {
    fn from(s: FileReaderSync) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn read_as_array_buffer(&self, blob: Blob) -> jsbind::ArrayBuffer {
        self.inner
            .call("readAsArrayBuffer", &[blob.into()])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl FileReaderSync {
    pub fn read_as_binary_string(&self, blob: Blob) -> jsbind::DOMString {
        self.inner
            .call("readAsBinaryString", &[blob.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl FileReaderSync {
    pub fn read_as_text0(&self, blob: Blob) -> jsbind::DOMString {
        self.inner
            .call("readAsText", &[blob.into()])
            .as_::<jsbind::DOMString>()
    }

    pub fn read_as_text1(&self, blob: Blob, encoding: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("readAsText", &[blob.into(), encoding.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl FileReaderSync {
    pub fn read_as_data_url(&self, blob: Blob) -> jsbind::DOMString {
        self.inner
            .call("readAsDataURL", &[blob.into()])
            .as_::<jsbind::DOMString>()
    }
}
