use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SecurityPolicyViolationEventInit {
    inner: Any,
}
impl FromVal for SecurityPolicyViolationEventInit {
    fn from_val(v: &Any) -> Self {
        SecurityPolicyViolationEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SecurityPolicyViolationEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SecurityPolicyViolationEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SecurityPolicyViolationEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SecurityPolicyViolationEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SecurityPolicyViolationEventInit> for Any {
    fn from(s: SecurityPolicyViolationEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SecurityPolicyViolationEventInit> for Any {
    fn from(s: &SecurityPolicyViolationEventInit) -> Any {
        s.inner.clone()
    }
}

impl SecurityPolicyViolationEventInit {
    pub fn document_uri(&self) -> JsString {
        self.inner.get("documentURI").as_::<JsString>()
    }

    pub fn set_document_uri(&mut self, value: &JsString) {
        self.inner.set("documentURI", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn referrer(&self) -> JsString {
        self.inner.get("referrer").as_::<JsString>()
    }

    pub fn set_referrer(&mut self, value: &JsString) {
        self.inner.set("referrer", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn blocked_uri(&self) -> JsString {
        self.inner.get("blockedURI").as_::<JsString>()
    }

    pub fn set_blocked_uri(&mut self, value: &JsString) {
        self.inner.set("blockedURI", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn violated_directive(&self) -> JsString {
        self.inner.get("violatedDirective").as_::<JsString>()
    }

    pub fn set_violated_directive(&mut self, value: &JsString) {
        self.inner.set("violatedDirective", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn effective_directive(&self) -> JsString {
        self.inner.get("effectiveDirective").as_::<JsString>()
    }

    pub fn set_effective_directive(&mut self, value: &JsString) {
        self.inner.set("effectiveDirective", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn original_policy(&self) -> JsString {
        self.inner.get("originalPolicy").as_::<JsString>()
    }

    pub fn set_original_policy(&mut self, value: &JsString) {
        self.inner.set("originalPolicy", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn source_file(&self) -> JsString {
        self.inner.get("sourceFile").as_::<JsString>()
    }

    pub fn set_source_file(&mut self, value: &JsString) {
        self.inner.set("sourceFile", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn sample(&self) -> JsString {
        self.inner.get("sample").as_::<JsString>()
    }

    pub fn set_sample(&mut self, value: &JsString) {
        self.inner.set("sample", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.inner
            .get("disposition")
            .as_::<SecurityPolicyViolationEventDisposition>()
    }

    pub fn set_disposition(&mut self, value: &SecurityPolicyViolationEventDisposition) {
        self.inner.set("disposition", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn status_code(&self) -> u16 {
        self.inner.get("statusCode").as_::<u16>()
    }

    pub fn set_status_code(&mut self, value: u16) {
        self.inner.set("statusCode", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }

    pub fn set_line_number(&mut self, value: u32) {
        self.inner.set("lineNumber", value);
    }
}
impl SecurityPolicyViolationEventInit {
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }

    pub fn set_column_number(&mut self, value: u32) {
        self.inner.set("columnNumber", value);
    }
}
