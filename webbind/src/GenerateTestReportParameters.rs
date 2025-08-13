use super::*;




/// The GenerateTestReportParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GenerateTestReportParameters {
    inner: Any,
}

impl FromVal for GenerateTestReportParameters {
    fn from_val(v: &Any) -> Self {
        GenerateTestReportParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GenerateTestReportParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GenerateTestReportParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GenerateTestReportParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GenerateTestReportParameters {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GenerateTestReportParameters> for Any {
    fn from(s: GenerateTestReportParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GenerateTestReportParameters> for Any {
    fn from(s: &GenerateTestReportParameters) -> Any {
        s.inner.clone()
    }
}

impl GenerateTestReportParameters {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
impl GenerateTestReportParameters {
    /// Getter of the `group` attribute.
    pub fn group(&self) -> JsString {
        self.inner.get("group").as_::<JsString>()
    }

    /// Setter of the `group` attribute.
    pub fn set_group(&mut self, value: &JsString) {
        self.inner.set("group", value);
    }
}
