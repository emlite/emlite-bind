use super::*;




/// The FilePropertyBag dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FilePropertyBag {
    inner: Any,
}

impl FromVal for FilePropertyBag {
    fn from_val(v: &Any) -> Self {
        FilePropertyBag { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FilePropertyBag {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FilePropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FilePropertyBag {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FilePropertyBag {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FilePropertyBag> for Any {
    fn from(s: FilePropertyBag) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FilePropertyBag> for Any {
    fn from(s: &FilePropertyBag) -> Any {
        s.inner.clone()
    }
}

impl FilePropertyBag {
    /// Getter of the `lastModified` attribute.
    pub fn last_modified(&self) -> i64 {
        self.inner.get("lastModified").as_::<i64>()
    }

    /// Setter of the `lastModified` attribute.
    pub fn set_last_modified(&mut self, value: i64) {
        self.inner.set("lastModified", value);
    }
}
