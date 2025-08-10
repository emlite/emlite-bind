use super::*;

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
    pub fn document_url(&self) -> JsString {
        self.inner.get("documentURL").as_::<JsString>()
    }

    pub fn set_document_url(&mut self, value: &JsString) {
        self.inner.set("documentURL", value);
    }
}
impl CSPViolationReportBody {
    pub fn referrer(&self) -> JsString {
        self.inner.get("referrer").as_::<JsString>()
    }

    pub fn set_referrer(&mut self, value: &JsString) {
        self.inner.set("referrer", value);
    }
}
impl CSPViolationReportBody {
    pub fn blocked_url(&self) -> JsString {
        self.inner.get("blockedURL").as_::<JsString>()
    }

    pub fn set_blocked_url(&mut self, value: &JsString) {
        self.inner.set("blockedURL", value);
    }
}
impl CSPViolationReportBody {
    pub fn effective_directive(&self) -> JsString {
        self.inner.get("effectiveDirective").as_::<JsString>()
    }

    pub fn set_effective_directive(&mut self, value: &JsString) {
        self.inner.set("effectiveDirective", value);
    }
}
impl CSPViolationReportBody {
    pub fn original_policy(&self) -> JsString {
        self.inner.get("originalPolicy").as_::<JsString>()
    }

    pub fn set_original_policy(&mut self, value: &JsString) {
        self.inner.set("originalPolicy", value);
    }
}
impl CSPViolationReportBody {
    pub fn source_file(&self) -> JsString {
        self.inner.get("sourceFile").as_::<JsString>()
    }

    pub fn set_source_file(&mut self, value: &JsString) {
        self.inner.set("sourceFile", value);
    }
}
impl CSPViolationReportBody {
    pub fn sample(&self) -> JsString {
        self.inner.get("sample").as_::<JsString>()
    }

    pub fn set_sample(&mut self, value: &JsString) {
        self.inner.set("sample", value);
    }
}
impl CSPViolationReportBody {
    pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.inner
            .get("disposition")
            .as_::<SecurityPolicyViolationEventDisposition>()
    }

    pub fn set_disposition(&mut self, value: &SecurityPolicyViolationEventDisposition) {
        self.inner.set("disposition", value);
    }
}
impl CSPViolationReportBody {
    pub fn status_code(&self) -> u16 {
        self.inner.get("statusCode").as_::<u16>()
    }

    pub fn set_status_code(&mut self, value: u16) {
        self.inner.set("statusCode", value);
    }
}
impl CSPViolationReportBody {
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }

    pub fn set_line_number(&mut self, value: u32) {
        self.inner.set("lineNumber", value);
    }
}
impl CSPViolationReportBody {
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }

    pub fn set_column_number(&mut self, value: u32) {
        self.inner.set("columnNumber", value);
    }
}
