use super::*;




/// The InterestGroupBiddingAndScoringScriptRunnerGlobalScope class.
/// [`InterestGroupBiddingAndScoringScriptRunnerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingAndScoringScriptRunnerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    inner: InterestGroupScriptRunnerGlobalScope,
}

impl FromVal for InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    fn from_val(v: &Any) -> Self {
        InterestGroupBiddingAndScoringScriptRunnerGlobalScope { inner: InterestGroupScriptRunnerGlobalScope::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<InterestGroupBiddingAndScoringScriptRunnerGlobalScope> for Any {
    fn from(s: InterestGroupBiddingAndScoringScriptRunnerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InterestGroupBiddingAndScoringScriptRunnerGlobalScope> for Any {
    fn from(s: &InterestGroupBiddingAndScoringScriptRunnerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(InterestGroupBiddingAndScoringScriptRunnerGlobalScope);


impl InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    /// Getter of the `forDebuggingOnly` attribute.
    /// [`InterestGroupBiddingAndScoringScriptRunnerGlobalScope.forDebuggingOnly`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingAndScoringScriptRunnerGlobalScope/forDebuggingOnly)
    pub fn for_debugging_only(&self) -> ForDebuggingOnly {
        self.inner.get("forDebuggingOnly").as_::<ForDebuggingOnly>()
    }

}
impl InterestGroupBiddingAndScoringScriptRunnerGlobalScope {
    /// Getter of the `realTimeReporting` attribute.
    /// [`InterestGroupBiddingAndScoringScriptRunnerGlobalScope.realTimeReporting`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupBiddingAndScoringScriptRunnerGlobalScope/realTimeReporting)
    pub fn real_time_reporting(&self) -> RealTimeReporting {
        self.inner.get("realTimeReporting").as_::<RealTimeReporting>()
    }

}
