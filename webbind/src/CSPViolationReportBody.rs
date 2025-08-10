use super::*;

/// The CSPViolationReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSPViolationReportBody {
    inner: Any,
}

impl FromVal for CSPViolationReportBody {
    fn from_val(v: &Any) -> Self {
        CSPViolationReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSPViolationReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSPViolationReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSPViolationReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSPViolationReportBody {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSPViolationReportBody> for Any {
    fn from(s: CSPViolationReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSPViolationReportBody> for Any {
    fn from(s: &CSPViolationReportBody) -> Any {
        s.inner.clone()
    }
}

impl CSPViolationReportBody {
    /// Getter of the `documentURL` attribute.
    pub fn document_url(&self) -> JsString {
        self.inner.get("documentURL").as_::<JsString>()
    }

    /// Setter of the `documentURL` attribute.
    pub fn set_document_url(&mut self, value: &JsString) {
        self.inner.set("documentURL", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `referrer` attribute.
    pub fn referrer(&self) -> JsString {
        self.inner.get("referrer").as_::<JsString>()
    }

    /// Setter of the `referrer` attribute.
    pub fn set_referrer(&mut self, value: &JsString) {
        self.inner.set("referrer", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `blockedURL` attribute.
    pub fn blocked_url(&self) -> JsString {
        self.inner.get("blockedURL").as_::<JsString>()
    }

    /// Setter of the `blockedURL` attribute.
    pub fn set_blocked_url(&mut self, value: &JsString) {
        self.inner.set("blockedURL", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `effectiveDirective` attribute.
    pub fn effective_directive(&self) -> JsString {
        self.inner.get("effectiveDirective").as_::<JsString>()
    }

    /// Setter of the `effectiveDirective` attribute.
    pub fn set_effective_directive(&mut self, value: &JsString) {
        self.inner.set("effectiveDirective", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `originalPolicy` attribute.
    pub fn original_policy(&self) -> JsString {
        self.inner.get("originalPolicy").as_::<JsString>()
    }

    /// Setter of the `originalPolicy` attribute.
    pub fn set_original_policy(&mut self, value: &JsString) {
        self.inner.set("originalPolicy", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `sourceFile` attribute.
    pub fn source_file(&self) -> JsString {
        self.inner.get("sourceFile").as_::<JsString>()
    }

    /// Setter of the `sourceFile` attribute.
    pub fn set_source_file(&mut self, value: &JsString) {
        self.inner.set("sourceFile", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `sample` attribute.
    pub fn sample(&self) -> JsString {
        self.inner.get("sample").as_::<JsString>()
    }

    /// Setter of the `sample` attribute.
    pub fn set_sample(&mut self, value: &JsString) {
        self.inner.set("sample", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `disposition` attribute.
    pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.inner
            .get("disposition")
            .as_::<SecurityPolicyViolationEventDisposition>()
    }

    /// Setter of the `disposition` attribute.
    pub fn set_disposition(&mut self, value: &SecurityPolicyViolationEventDisposition) {
        self.inner.set("disposition", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `statusCode` attribute.
    pub fn status_code(&self) -> u16 {
        self.inner.get("statusCode").as_::<u16>()
    }

    /// Setter of the `statusCode` attribute.
    pub fn set_status_code(&mut self, value: u16) {
        self.inner.set("statusCode", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `lineNumber` attribute.
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }

    /// Setter of the `lineNumber` attribute.
    pub fn set_line_number(&mut self, value: u32) {
        self.inner.set("lineNumber", value);
    }
}
impl CSPViolationReportBody {
    /// Getter of the `columnNumber` attribute.
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }

    /// Setter of the `columnNumber` attribute.
    pub fn set_column_number(&mut self, value: u32) {
        self.inner.set("columnNumber", value);
    }
}
