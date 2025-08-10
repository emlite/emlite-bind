use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PermissionsPolicyViolationReportBody {
    inner: Any,
}
impl FromVal for PermissionsPolicyViolationReportBody {
    fn from_val(v: &Any) -> Self {
        PermissionsPolicyViolationReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PermissionsPolicyViolationReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PermissionsPolicyViolationReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PermissionsPolicyViolationReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PermissionsPolicyViolationReportBody {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PermissionsPolicyViolationReportBody> for Any {
    fn from(s: PermissionsPolicyViolationReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PermissionsPolicyViolationReportBody> for Any {
    fn from(s: &PermissionsPolicyViolationReportBody) -> Any {
        s.inner.clone()
    }
}

impl PermissionsPolicyViolationReportBody {
    pub fn feature_id(&self) -> JsString {
        self.inner.get("featureId").as_::<JsString>()
    }

    pub fn set_feature_id(&mut self, value: &JsString) {
        self.inner.set("featureId", value);
    }
}
impl PermissionsPolicyViolationReportBody {
    pub fn source_file(&self) -> JsString {
        self.inner.get("sourceFile").as_::<JsString>()
    }

    pub fn set_source_file(&mut self, value: &JsString) {
        self.inner.set("sourceFile", value);
    }
}
impl PermissionsPolicyViolationReportBody {
    pub fn line_number(&self) -> i32 {
        self.inner.get("lineNumber").as_::<i32>()
    }

    pub fn set_line_number(&mut self, value: i32) {
        self.inner.set("lineNumber", value);
    }
}
impl PermissionsPolicyViolationReportBody {
    pub fn column_number(&self) -> i32 {
        self.inner.get("columnNumber").as_::<i32>()
    }

    pub fn set_column_number(&mut self, value: i32) {
        self.inner.set("columnNumber", value);
    }
}
impl PermissionsPolicyViolationReportBody {
    pub fn disposition(&self) -> JsString {
        self.inner.get("disposition").as_::<JsString>()
    }

    pub fn set_disposition(&mut self, value: &JsString) {
        self.inner.set("disposition", value);
    }
}
impl PermissionsPolicyViolationReportBody {
    pub fn allow_attribute(&self) -> JsString {
        self.inner.get("allowAttribute").as_::<JsString>()
    }

    pub fn set_allow_attribute(&mut self, value: &JsString) {
        self.inner.set("allowAttribute", value);
    }
}
impl PermissionsPolicyViolationReportBody {
    pub fn src_attribute(&self) -> JsString {
        self.inner.get("srcAttribute").as_::<JsString>()
    }

    pub fn set_src_attribute(&mut self, value: &JsString) {
        self.inner.set("srcAttribute", value);
    }
}
