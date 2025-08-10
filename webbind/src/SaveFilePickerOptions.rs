use super::*;

/// The SaveFilePickerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SaveFilePickerOptions {
    inner: Any,
}

impl FromVal for SaveFilePickerOptions {
    fn from_val(v: &Any) -> Self {
        SaveFilePickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SaveFilePickerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SaveFilePickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SaveFilePickerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SaveFilePickerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SaveFilePickerOptions> for Any {
    fn from(s: SaveFilePickerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SaveFilePickerOptions> for Any {
    fn from(s: &SaveFilePickerOptions) -> Any {
        s.inner.clone()
    }
}

impl SaveFilePickerOptions {
    /// Getter of the `suggestedName` attribute.
    pub fn suggested_name(&self) -> JsString {
        self.inner.get("suggestedName").as_::<JsString>()
    }

    /// Setter of the `suggestedName` attribute.
    pub fn set_suggested_name(&mut self, value: &JsString) {
        self.inner.set("suggestedName", value);
    }
}
