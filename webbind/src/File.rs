use super::*;

/// The File class.
/// [`File`](https://developer.mozilla.org/en-US/docs/Web/API/File)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct File {
    inner: Blob,
}
impl FromVal for File {
    fn from_val(v: &Any) -> Self {
        File {
            inner: Blob::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for File {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for File {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<File> for Any {
    fn from(s: File) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&File> for Any {
    fn from(s: &File) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(File);

impl File {
    /// The `new File(..)` constructor, creating a new File instance
    pub fn new0(file_bits: &Sequence<Any>, file_name: &USVString) -> File {
        Self {
            inner: Any::global("File")
                .new(&[file_bits.into(), file_name.into()])
                .as_::<Blob>(),
        }
    }

    /// The `new File(..)` constructor, creating a new File instance
    pub fn new1(file_bits: &Sequence<Any>, file_name: &USVString, options: &Any) -> File {
        Self {
            inner: Any::global("File")
                .new(&[file_bits.into(), file_name.into(), options.into()])
                .as_::<Blob>(),
        }
    }
}
impl File {
    /// Getter of the `name` attribute.
    /// [`File.name`](https://developer.mozilla.org/en-US/docs/Web/API/File/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl File {
    /// Getter of the `lastModified` attribute.
    /// [`File.lastModified`](https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified)
    pub fn last_modified(&self) -> i64 {
        self.inner.get("lastModified").as_::<i64>()
    }
}
impl File {
    /// Getter of the `webkitRelativePath` attribute.
    /// [`File.webkitRelativePath`](https://developer.mozilla.org/en-US/docs/Web/API/File/webkitRelativePath)
    pub fn webkit_relative_path(&self) -> USVString {
        self.inner.get("webkitRelativePath").as_::<USVString>()
    }
}
