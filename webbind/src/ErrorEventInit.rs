use super::*;

/// The ErrorEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ErrorEventInit {
    inner: Any,
}

impl FromVal for ErrorEventInit {
    fn from_val(v: &Any) -> Self {
        ErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ErrorEventInit> for Any {
    fn from(s: ErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ErrorEventInit> for Any {
    fn from(s: &ErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl ErrorEventInit {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
impl ErrorEventInit {
    /// Getter of the `filename` attribute.
    pub fn filename(&self) -> JsString {
        self.inner.get("filename").as_::<JsString>()
    }

    /// Setter of the `filename` attribute.
    pub fn set_filename(&mut self, value: &JsString) {
        self.inner.set("filename", value);
    }
}
impl ErrorEventInit {
    /// Getter of the `lineno` attribute.
    pub fn lineno(&self) -> u32 {
        self.inner.get("lineno").as_::<u32>()
    }

    /// Setter of the `lineno` attribute.
    pub fn set_lineno(&mut self, value: u32) {
        self.inner.set("lineno", value);
    }
}
impl ErrorEventInit {
    /// Getter of the `colno` attribute.
    pub fn colno(&self) -> u32 {
        self.inner.get("colno").as_::<u32>()
    }

    /// Setter of the `colno` attribute.
    pub fn set_colno(&mut self, value: u32) {
        self.inner.set("colno", value);
    }
}
impl ErrorEventInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> Any {
        self.inner.get("error").as_::<Any>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &Any) {
        self.inner.set("error", value);
    }
}
