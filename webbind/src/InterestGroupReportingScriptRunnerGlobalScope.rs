use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupReportingScriptRunnerGlobalScope {
    inner: InterestGroupScriptRunnerGlobalScope,
}
impl FromVal for InterestGroupReportingScriptRunnerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        InterestGroupReportingScriptRunnerGlobalScope {
            inner: InterestGroupScriptRunnerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for InterestGroupReportingScriptRunnerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InterestGroupReportingScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<InterestGroupReportingScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: InterestGroupReportingScriptRunnerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&InterestGroupReportingScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: &InterestGroupReportingScriptRunnerGlobalScope) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(InterestGroupReportingScriptRunnerGlobalScope);

impl InterestGroupReportingScriptRunnerGlobalScope {
    pub fn send_report_to(&self, url: DOMString) -> Undefined {
        self.inner
            .call("sendReportTo", &[url.into()])
            .as_::<Undefined>()
    }
}
impl InterestGroupReportingScriptRunnerGlobalScope {
    pub fn register_ad_beacon(&self, map: Record<DOMString, USVString>) -> Undefined {
        self.inner
            .call("registerAdBeacon", &[map.into()])
            .as_::<Undefined>()
    }
}
impl InterestGroupReportingScriptRunnerGlobalScope {
    pub fn register_ad_macro(&self, name: DOMString, value: USVString) -> Undefined {
        self.inner
            .call("registerAdMacro", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
