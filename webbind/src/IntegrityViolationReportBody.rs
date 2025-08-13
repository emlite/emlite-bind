use super::*;




/// The IntegrityViolationReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntegrityViolationReportBody {
    inner: Any,
}

impl FromVal for IntegrityViolationReportBody {
    fn from_val(v: &Any) -> Self {
        IntegrityViolationReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IntegrityViolationReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IntegrityViolationReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IntegrityViolationReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IntegrityViolationReportBody {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IntegrityViolationReportBody> for Any {
    fn from(s: IntegrityViolationReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IntegrityViolationReportBody> for Any {
    fn from(s: &IntegrityViolationReportBody) -> Any {
        s.inner.clone()
    }
}

impl IntegrityViolationReportBody {
    /// Getter of the `documentURL` attribute.
    pub fn document_url(&self) -> JsString {
        self.inner.get("documentURL").as_::<JsString>()
    }

    /// Setter of the `documentURL` attribute.
    pub fn set_document_url(&mut self, value: &JsString) {
        self.inner.set("documentURL", value);
    }
}
impl IntegrityViolationReportBody {
    /// Getter of the `blockedURL` attribute.
    pub fn blocked_url(&self) -> JsString {
        self.inner.get("blockedURL").as_::<JsString>()
    }

    /// Setter of the `blockedURL` attribute.
    pub fn set_blocked_url(&mut self, value: &JsString) {
        self.inner.set("blockedURL", value);
    }
}
impl IntegrityViolationReportBody {
    /// Getter of the `destination` attribute.
    pub fn destination(&self) -> JsString {
        self.inner.get("destination").as_::<JsString>()
    }

    /// Setter of the `destination` attribute.
    pub fn set_destination(&mut self, value: &JsString) {
        self.inner.set("destination", value);
    }
}
impl IntegrityViolationReportBody {
    /// Getter of the `reportOnly` attribute.
    pub fn report_only(&self) -> bool {
        self.inner.get("reportOnly").as_::<bool>()
    }

    /// Setter of the `reportOnly` attribute.
    pub fn set_report_only(&mut self, value: bool) {
        self.inner.set("reportOnly", value);
    }
}
