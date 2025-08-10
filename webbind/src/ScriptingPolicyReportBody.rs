use super::*;

/// The ScriptingPolicyReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScriptingPolicyReportBody {
    inner: Any,
}

impl FromVal for ScriptingPolicyReportBody {
    fn from_val(v: &Any) -> Self {
        ScriptingPolicyReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ScriptingPolicyReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScriptingPolicyReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScriptingPolicyReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScriptingPolicyReportBody {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ScriptingPolicyReportBody> for Any {
    fn from(s: ScriptingPolicyReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScriptingPolicyReportBody> for Any {
    fn from(s: &ScriptingPolicyReportBody) -> Any {
        s.inner.clone()
    }
}

impl ScriptingPolicyReportBody {
    /// Getter of the `violationType` attribute.
    pub fn violation_type(&self) -> JsString {
        self.inner.get("violationType").as_::<JsString>()
    }

    /// Setter of the `violationType` attribute.
    pub fn set_violation_type(&mut self, value: &JsString) {
        self.inner.set("violationType", value);
    }
}
impl ScriptingPolicyReportBody {
    /// Getter of the `violationURL` attribute.
    pub fn violation_url(&self) -> JsString {
        self.inner.get("violationURL").as_::<JsString>()
    }

    /// Setter of the `violationURL` attribute.
    pub fn set_violation_url(&mut self, value: &JsString) {
        self.inner.set("violationURL", value);
    }
}
impl ScriptingPolicyReportBody {
    /// Getter of the `violationSample` attribute.
    pub fn violation_sample(&self) -> JsString {
        self.inner.get("violationSample").as_::<JsString>()
    }

    /// Setter of the `violationSample` attribute.
    pub fn set_violation_sample(&mut self, value: &JsString) {
        self.inner.set("violationSample", value);
    }
}
impl ScriptingPolicyReportBody {
    /// Getter of the `lineno` attribute.
    pub fn lineno(&self) -> u32 {
        self.inner.get("lineno").as_::<u32>()
    }

    /// Setter of the `lineno` attribute.
    pub fn set_lineno(&mut self, value: u32) {
        self.inner.set("lineno", value);
    }
}
impl ScriptingPolicyReportBody {
    /// Getter of the `colno` attribute.
    pub fn colno(&self) -> u32 {
        self.inner.get("colno").as_::<u32>()
    }

    /// Setter of the `colno` attribute.
    pub fn set_colno(&mut self, value: u32) {
        self.inner.set("colno", value);
    }
}
