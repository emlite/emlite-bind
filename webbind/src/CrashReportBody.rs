use super::*;




/// The CrashReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CrashReportBody {
    inner: Any,
}

impl FromVal for CrashReportBody {
    fn from_val(v: &Any) -> Self {
        CrashReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CrashReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CrashReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CrashReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CrashReportBody {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CrashReportBody> for Any {
    fn from(s: CrashReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CrashReportBody> for Any {
    fn from(s: &CrashReportBody) -> Any {
        s.inner.clone()
    }
}

impl CrashReportBody {
    /// Getter of the `reason` attribute.
    pub fn reason(&self) -> JsString {
        self.inner.get("reason").as_::<JsString>()
    }

    /// Setter of the `reason` attribute.
    pub fn set_reason(&mut self, value: &JsString) {
        self.inner.set("reason", value);
    }
}
impl CrashReportBody {
    /// Getter of the `stack` attribute.
    pub fn stack(&self) -> JsString {
        self.inner.get("stack").as_::<JsString>()
    }

    /// Setter of the `stack` attribute.
    pub fn set_stack(&mut self, value: &JsString) {
        self.inner.set("stack", value);
    }
}
impl CrashReportBody {
    /// Getter of the `is_top_level` attribute.
    pub fn is_top_level(&self) -> bool {
        self.inner.get("is_top_level").as_::<bool>()
    }

    /// Setter of the `is_top_level` attribute.
    pub fn set_is_top_level(&mut self, value: bool) {
        self.inner.set("is_top_level", value);
    }
}
impl CrashReportBody {
    /// Getter of the `page_visibility` attribute.
    pub fn page_visibility(&self) -> DocumentVisibilityState {
        self.inner.get("page_visibility").as_::<DocumentVisibilityState>()
    }

    /// Setter of the `page_visibility` attribute.
    pub fn set_page_visibility(&mut self, value: &DocumentVisibilityState) {
        self.inner.set("page_visibility", value);
    }
}
