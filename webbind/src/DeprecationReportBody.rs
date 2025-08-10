use super::*;

/// The DeprecationReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeprecationReportBody {
    inner: Any,
}

impl FromVal for DeprecationReportBody {
    fn from_val(v: &Any) -> Self {
        DeprecationReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DeprecationReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DeprecationReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DeprecationReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DeprecationReportBody {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DeprecationReportBody> for Any {
    fn from(s: DeprecationReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DeprecationReportBody> for Any {
    fn from(s: &DeprecationReportBody) -> Any {
        s.inner.clone()
    }
}

impl DeprecationReportBody {
    /// Getter of the `id` attribute.
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    /// Setter of the `id` attribute.
    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
impl DeprecationReportBody {
    /// Getter of the `anticipatedRemoval` attribute.
    pub fn anticipated_removal(&self) -> Object {
        self.inner.get("anticipatedRemoval").as_::<Object>()
    }

    /// Setter of the `anticipatedRemoval` attribute.
    pub fn set_anticipated_removal(&mut self, value: &Object) {
        self.inner.set("anticipatedRemoval", value);
    }
}
impl DeprecationReportBody {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
impl DeprecationReportBody {
    /// Getter of the `sourceFile` attribute.
    pub fn source_file(&self) -> JsString {
        self.inner.get("sourceFile").as_::<JsString>()
    }

    /// Setter of the `sourceFile` attribute.
    pub fn set_source_file(&mut self, value: &JsString) {
        self.inner.set("sourceFile", value);
    }
}
impl DeprecationReportBody {
    /// Getter of the `lineNumber` attribute.
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }

    /// Setter of the `lineNumber` attribute.
    pub fn set_line_number(&mut self, value: u32) {
        self.inner.set("lineNumber", value);
    }
}
impl DeprecationReportBody {
    /// Getter of the `columnNumber` attribute.
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }

    /// Setter of the `columnNumber` attribute.
    pub fn set_column_number(&mut self, value: u32) {
        self.inner.set("columnNumber", value);
    }
}
