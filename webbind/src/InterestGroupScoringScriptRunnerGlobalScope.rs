use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterestGroupScoringScriptRunnerGlobalScope {
    inner: InterestGroupBiddingAndScoringScriptRunnerGlobalScope,
}
impl FromVal for InterestGroupScoringScriptRunnerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        InterestGroupScoringScriptRunnerGlobalScope {
            inner: InterestGroupBiddingAndScoringScriptRunnerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InterestGroupScoringScriptRunnerGlobalScope {
    type Target = InterestGroupBiddingAndScoringScriptRunnerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InterestGroupScoringScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<InterestGroupScoringScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: InterestGroupScoringScriptRunnerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
