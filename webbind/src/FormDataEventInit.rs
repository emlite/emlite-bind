use super::*;

/// The FormDataEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FormDataEventInit {
    inner: Any,
}

impl FromVal for FormDataEventInit {
    fn from_val(v: &Any) -> Self {
        FormDataEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FormDataEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FormDataEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FormDataEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FormDataEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FormDataEventInit> for Any {
    fn from(s: FormDataEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FormDataEventInit> for Any {
    fn from(s: &FormDataEventInit) -> Any {
        s.inner.clone()
    }
}

impl FormDataEventInit {
    /// Getter of the `formData` attribute.
    pub fn form_data(&self) -> FormData {
        self.inner.get("formData").as_::<FormData>()
    }

    /// Setter of the `formData` attribute.
    pub fn set_form_data(&mut self, value: &FormData) {
        self.inner.set("formData", value);
    }
}
