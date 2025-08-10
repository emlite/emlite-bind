use super::*;

/// The InterventionReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterventionReportBody {
    inner: Any,
}

impl FromVal for InterventionReportBody {
    fn from_val(v: &Any) -> Self {
        InterventionReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for InterventionReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InterventionReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InterventionReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InterventionReportBody {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<InterventionReportBody> for Any {
    fn from(s: InterventionReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InterventionReportBody> for Any {
    fn from(s: &InterventionReportBody) -> Any {
        s.inner.clone()
    }
}

impl InterventionReportBody {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl InterventionReportBody {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
impl InterventionReportBody {
    /// Getter of the `sourceFile` attribute.
    pub fn source_file(&self) -> JsString {
        self.inner.get("sourceFile").as_::<JsString>()
    }

    /// Setter of the `sourceFile` attribute.
    pub fn set_source_file(&mut self, value: &JsString) {
        self.inner.set("sourceFile", value);
    }
}
impl InterventionReportBody {
    /// Getter of the `lineNumber` attribute.
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }

    /// Setter of the `lineNumber` attribute.
    pub fn set_line_number(&mut self, value: u32) {
        self.inner.set("lineNumber", value);
    }
}
impl InterventionReportBody {
    /// Getter of the `columnNumber` attribute.
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }

    /// Setter of the `columnNumber` attribute.
    pub fn set_column_number(&mut self, value: u32) {
        self.inner.set("columnNumber", value);
    }
}
