use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct File {
    inner: Blob,
}
impl FromVal for File {
    fn from_val(v: &emlite::Val) -> Self {
        File {
            inner: Blob::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for File {
    type Target = Blob;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for File {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for File {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for File {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<File> for emlite::Val {
    fn from(s: File) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&File> for emlite::Val {
    fn from(s: &File) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(File);

impl File {
    pub fn new0(file_bits: Sequence<Any>, file_name: USVString) -> File {
        Self {
            inner: emlite::Val::global("File")
                .new(&[file_bits.into(), file_name.into()])
                .as_::<Blob>(),
        }
    }

    pub fn new1(file_bits: Sequence<Any>, file_name: USVString, options: Any) -> File {
        Self {
            inner: emlite::Val::global("File")
                .new(&[file_bits.into(), file_name.into(), options.into()])
                .as_::<Blob>(),
        }
    }
}
impl File {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl File {
    pub fn last_modified(&self) -> i64 {
        self.inner.get("lastModified").as_::<i64>()
    }
}
impl File {
    pub fn webkit_relative_path(&self) -> USVString {
        self.inner.get("webkitRelativePath").as_::<USVString>()
    }
}
