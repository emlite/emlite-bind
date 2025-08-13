use super::*;




/// The InterestGroupScriptRunnerGlobalScope class.
/// [`InterestGroupScriptRunnerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupScriptRunnerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InterestGroupScriptRunnerGlobalScope {
    inner: Any,
}

impl FromVal for InterestGroupScriptRunnerGlobalScope {
    fn from_val(v: &Any) -> Self {
        InterestGroupScriptRunnerGlobalScope { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for InterestGroupScriptRunnerGlobalScope {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InterestGroupScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InterestGroupScriptRunnerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InterestGroupScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<InterestGroupScriptRunnerGlobalScope> for Any {
    fn from(s: InterestGroupScriptRunnerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InterestGroupScriptRunnerGlobalScope> for Any {
    fn from(s: &InterestGroupScriptRunnerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(InterestGroupScriptRunnerGlobalScope);


impl InterestGroupScriptRunnerGlobalScope {
    /// Getter of the `privateAggregation` attribute.
    /// [`InterestGroupScriptRunnerGlobalScope.privateAggregation`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupScriptRunnerGlobalScope/privateAggregation)
    pub fn private_aggregation(&self) -> PrivateAggregation {
        self.inner.get("privateAggregation").as_::<PrivateAggregation>()
    }

}
impl InterestGroupScriptRunnerGlobalScope {
    /// Getter of the `protectedAudience` attribute.
    /// [`InterestGroupScriptRunnerGlobalScope.protectedAudience`](https://developer.mozilla.org/en-US/docs/Web/API/InterestGroupScriptRunnerGlobalScope/protectedAudience)
    pub fn protected_audience(&self) -> ProtectedAudienceUtilities {
        self.inner.get("protectedAudience").as_::<ProtectedAudienceUtilities>()
    }

}
