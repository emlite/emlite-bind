use super::*;

/// The InterestGroupBiddingScriptRunnerGlobalScope class.
/// [`InterestGroupBiddingScriptRunnerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingScriptRunnerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupBiddingScriptRunnerGlobalScope {
    inner: InterestGroupBiddingAndScoringScriptRunnerGlobalScope,
}
impl FromVal for InterestGroupBiddingScriptRunnerGlobalScope {
    fn from_val(v: &Any) -> Self {
        InterestGroupBiddingScriptRunnerGlobalScope {
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
impl AsRef<Any> for InterestGroupBiddingScriptRunnerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for InterestGroupBiddingScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<InterestGroupBiddingScriptRunnerGlobalScope> for Any {
    fn from(s: InterestGroupBiddingScriptRunnerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&InterestGroupBiddingScriptRunnerGlobalScope> for Any {
    fn from(s: &InterestGroupBiddingScriptRunnerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(InterestGroupBiddingScriptRunnerGlobalScope);

impl InterestGroupBiddingScriptRunnerGlobalScope {
    /// The setBid method.
    /// [`InterestGroupBiddingScriptRunnerGlobalScope.setBid`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingScriptRunnerGlobalScope/setBid)
    pub fn set_bid0(&self) -> bool {
        self.inner.call("setBid", &[]).as_::<bool>()
    }
    /// The setBid method.
    /// [`InterestGroupBiddingScriptRunnerGlobalScope.setBid`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingScriptRunnerGlobalScope/setBid)
    pub fn set_bid1(&self, one_or_many_bids: &Any) -> bool {
        self.inner
            .call("setBid", &[one_or_many_bids.into()])
            .as_::<bool>()
    }
}
impl InterestGroupBiddingScriptRunnerGlobalScope {
    /// The setPriority method.
    /// [`InterestGroupBiddingScriptRunnerGlobalScope.setPriority`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingScriptRunnerGlobalScope/setPriority)
    pub fn set_priority(&self, priority: f64) -> Undefined {
        self.inner
            .call("setPriority", &[priority.into()])
            .as_::<Undefined>()
    }
}
impl InterestGroupBiddingScriptRunnerGlobalScope {
    /// The setPrioritySignalsOverride method.
    /// [`InterestGroupBiddingScriptRunnerGlobalScope.setPrioritySignalsOverride`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingScriptRunnerGlobalScope/setPrioritySignalsOverride)
    pub fn set_priority_signals_override0(&self, key: &DOMString) -> Undefined {
        self.inner
            .call("setPrioritySignalsOverride", &[key.into()])
            .as_::<Undefined>()
    }
    /// The setPrioritySignalsOverride method.
    /// [`InterestGroupBiddingScriptRunnerGlobalScope.setPrioritySignalsOverride`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingScriptRunnerGlobalScope/setPrioritySignalsOverride)
    pub fn set_priority_signals_override1(&self, key: &DOMString, priority: f64) -> Undefined {
        self.inner
            .call("setPrioritySignalsOverride", &[key.into(), priority.into()])
            .as_::<Undefined>()
    }
}
