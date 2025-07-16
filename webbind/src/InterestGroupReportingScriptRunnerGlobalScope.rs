use super::*;

/// The InterestGroupReportingScriptRunnerGlobalScope class.
/// [`InterestGroupReportingScriptRunnerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupReportingScriptRunnerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupReportingScriptRunnerGlobalScope {
    inner: InterestGroupScriptRunnerGlobalScope,
}
impl FromVal for InterestGroupReportingScriptRunnerGlobalScope {
    fn from_val(v: &Any) -> Self {
        InterestGroupReportingScriptRunnerGlobalScope {
            inner: InterestGroupScriptRunnerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InterestGroupReportingScriptRunnerGlobalScope {
    type Target = InterestGroupScriptRunnerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InterestGroupReportingScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for InterestGroupReportingScriptRunnerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for InterestGroupReportingScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<InterestGroupReportingScriptRunnerGlobalScope> for Any {
    fn from(s: InterestGroupReportingScriptRunnerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&InterestGroupReportingScriptRunnerGlobalScope> for Any {
    fn from(s: &InterestGroupReportingScriptRunnerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(InterestGroupReportingScriptRunnerGlobalScope);

impl InterestGroupReportingScriptRunnerGlobalScope {
    /// The sendReportTo method.
    /// [`InterestGroupReportingScriptRunnerGlobalScope.sendReportTo`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupReportingScriptRunnerGlobalScope/sendReportTo)
    pub fn send_report_to(&self, url: &str) -> Undefined {
        self.inner
            .call("sendReportTo", &[url.into()])
            .as_::<Undefined>()
    }
}
impl InterestGroupReportingScriptRunnerGlobalScope {
    /// The registerAdBeacon method.
    /// [`InterestGroupReportingScriptRunnerGlobalScope.registerAdBeacon`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupReportingScriptRunnerGlobalScope/registerAdBeacon)
    pub fn register_ad_beacon(&self, map: &Record<String, String>) -> Undefined {
        self.inner
            .call("registerAdBeacon", &[map.into()])
            .as_::<Undefined>()
    }
}
impl InterestGroupReportingScriptRunnerGlobalScope {
    /// The registerAdMacro method.
    /// [`InterestGroupReportingScriptRunnerGlobalScope.registerAdMacro`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupReportingScriptRunnerGlobalScope/registerAdMacro)
    pub fn register_ad_macro(&self, name: &str, value: &str) -> Undefined {
        self.inner
            .call("registerAdMacro", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
