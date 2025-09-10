use super::*;

/// The FileList class.
/// [`FileList`](https://developer.mozilla.org/en-US/docs/Web/API/FileList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FileList {
    inner: Any,
}

impl FromVal for FileList {
    fn from_val(v: &Any) -> Self {
        FileList {
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

impl core::ops::Deref for FileList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FileList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FileList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FileList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FileList> for Any {
    fn from(s: FileList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FileList> for Any {
    fn from(s: &FileList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FileList);

impl FileList {
    /// The item method.
    /// [`FileList.item`](https://developer.mozilla.org/en-US/docs/Web/API/FileList/item)
    pub fn item(&self, index: u32) -> File {
        self.inner.call("item", &[index.into()]).as_::<File>()
    }
}
impl FileList {
    /// Getter of the `length` attribute.
    /// [`FileList.length`](https://developer.mozilla.org/en-US/docs/Web/API/FileList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
