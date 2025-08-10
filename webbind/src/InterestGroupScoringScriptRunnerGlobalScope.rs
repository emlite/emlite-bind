use super::*;

/// The InterestGroupScoringScriptRunnerGlobalScope class.
/// [`InterestGroupScoringScriptRunnerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupScoringScriptRunnerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupScoringScriptRunnerGlobalScope {
    inner: InterestGroupBiddingAndScoringScriptRunnerGlobalScope,
}

impl FromVal for InterestGroupScoringScriptRunnerGlobalScope {
    fn from_val(v: &Any) -> Self {
        InterestGroupScoringScriptRunnerGlobalScope {
            inner: InterestGroupBiddingAndScoringScriptRunnerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for InterestGroupScoringScriptRunnerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InterestGroupScoringScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<InterestGroupScoringScriptRunnerGlobalScope> for Any {
    fn from(s: InterestGroupScoringScriptRunnerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InterestGroupScoringScriptRunnerGlobalScope> for Any {
    fn from(s: &InterestGroupScoringScriptRunnerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(InterestGroupScoringScriptRunnerGlobalScope);
