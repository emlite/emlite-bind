use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupBiddingScriptRunnerGlobalScope {
    inner: InterestGroupBiddingAndScoringScriptRunnerGlobalScope,
}
impl FromVal for InterestGroupBiddingScriptRunnerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        InterestGroupBiddingScriptRunnerGlobalScope {
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
impl core::ops::Deref for InterestGroupBiddingScriptRunnerGlobalScope {
    type Target = InterestGroupBiddingAndScoringScriptRunnerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InterestGroupBiddingScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for InterestGroupBiddingScriptRunnerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InterestGroupBiddingScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<InterestGroupBiddingScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: InterestGroupBiddingScriptRunnerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(InterestGroupBiddingScriptRunnerGlobalScope);

impl InterestGroupBiddingScriptRunnerGlobalScope {
    pub fn set_bid0(&self) -> bool {
        self.inner.call("setBid", &[]).as_::<bool>()
    }

    pub fn set_bid1(&self, one_or_many_bids: Any) -> bool {
        self.inner
            .call("setBid", &[one_or_many_bids.into()])
            .as_::<bool>()
    }
}
impl InterestGroupBiddingScriptRunnerGlobalScope {
    pub fn set_priority(&self, priority: f64) -> Undefined {
        self.inner
            .call("setPriority", &[priority.into()])
            .as_::<Undefined>()
    }
}
impl InterestGroupBiddingScriptRunnerGlobalScope {
    pub fn set_priority_signals_override0(&self, key: DOMString) -> Undefined {
        self.inner
            .call("setPrioritySignalsOverride", &[key.into()])
            .as_::<Undefined>()
    }

    pub fn set_priority_signals_override1(&self, key: DOMString, priority: f64) -> Undefined {
        self.inner
            .call("setPrioritySignalsOverride", &[key.into(), priority.into()])
            .as_::<Undefined>()
    }
}
