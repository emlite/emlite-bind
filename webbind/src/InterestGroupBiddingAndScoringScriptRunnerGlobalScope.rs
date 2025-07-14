use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    inner: InterestGroupScriptRunnerGlobalScope,
}
impl FromVal for InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
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
impl core::ops::Deref for InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    type Target = InterestGroupScriptRunnerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<InterestGroupBiddingAndScoringScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: InterestGroupBiddingAndScoringScriptRunnerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    pub fn for_debugging_only(&self) -> ForDebuggingOnly {
        self.inner.get("forDebuggingOnly").as_::<ForDebuggingOnly>()
    }
}
impl InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    pub fn real_time_reporting(&self) -> RealTimeReporting {
        self.inner
            .get("realTimeReporting")
            .as_::<RealTimeReporting>()
    }
}
