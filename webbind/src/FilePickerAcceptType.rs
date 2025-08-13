use super::*;




/// The FilePickerAcceptType dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FilePickerAcceptType {
    inner: Any,
}

impl FromVal for FilePickerAcceptType {
    fn from_val(v: &Any) -> Self {
        FilePickerAcceptType { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FilePickerAcceptType {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FilePickerAcceptType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FilePickerAcceptType {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FilePickerAcceptType {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FilePickerAcceptType> for Any {
    fn from(s: FilePickerAcceptType) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FilePickerAcceptType> for Any {
    fn from(s: &FilePickerAcceptType) -> Any {
        s.inner.clone()
    }
}

impl FilePickerAcceptType {
    /// Getter of the `description` attribute.
    pub fn description(&self) -> JsString {
        self.inner.get("description").as_::<JsString>()
    }

    /// Setter of the `description` attribute.
    pub fn set_description(&mut self, value: &JsString) {
        self.inner.set("description", value);
    }
}
impl FilePickerAcceptType {
    /// Getter of the `accept` attribute.
    pub fn accept(&self) -> Record<JsString, Any> {
        self.inner.get("accept").as_::<Record<JsString, Any>>()
    }

    /// Setter of the `accept` attribute.
    pub fn set_accept(&mut self, value: &Record<JsString, Any>) {
        self.inner.set("accept", value);
    }
}
