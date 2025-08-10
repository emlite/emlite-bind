use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FilePickerOptions {
    inner: Any,
}
impl FromVal for FilePickerOptions {
    fn from_val(v: &Any) -> Self {
        FilePickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FilePickerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FilePickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FilePickerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FilePickerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FilePickerOptions> for Any {
    fn from(s: FilePickerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FilePickerOptions> for Any {
    fn from(s: &FilePickerOptions) -> Any {
        s.inner.clone()
    }
}

impl FilePickerOptions {
    pub fn types(&self) -> TypedArray<FilePickerAcceptType> {
        self.inner
            .get("types")
            .as_::<TypedArray<FilePickerAcceptType>>()
    }

    pub fn set_types(&mut self, value: &TypedArray<FilePickerAcceptType>) {
        self.inner.set("types", value);
    }
}
impl FilePickerOptions {
    pub fn exclude_accept_all_option(&self) -> bool {
        self.inner.get("excludeAcceptAllOption").as_::<bool>()
    }

    pub fn set_exclude_accept_all_option(&mut self, value: bool) {
        self.inner.set("excludeAcceptAllOption", value);
    }
}
impl FilePickerOptions {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl FilePickerOptions {
    pub fn start_in(&self) -> Any {
        self.inner.get("startIn").as_::<Any>()
    }

    pub fn set_start_in(&mut self, value: &Any) {
        self.inner.set("startIn", value);
    }
}
