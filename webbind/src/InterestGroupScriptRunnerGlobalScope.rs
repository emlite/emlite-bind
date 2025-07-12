use super::*;

#[derive(Clone, Debug)]
pub struct InterestGroupScriptRunnerGlobalScope {
    inner: emlite::Val,
}
impl FromVal for InterestGroupScriptRunnerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        InterestGroupScriptRunnerGlobalScope {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for InterestGroupScriptRunnerGlobalScope {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for InterestGroupScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<InterestGroupScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: InterestGroupScriptRunnerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl InterestGroupScriptRunnerGlobalScope {
    pub fn private_aggregation(&self) -> PrivateAggregation {
        self.inner
            .get("privateAggregation")
            .as_::<PrivateAggregation>()
    }
}
impl InterestGroupScriptRunnerGlobalScope {
    pub fn protected_audience(&self) -> ProtectedAudienceUtilities {
        self.inner
            .get("protectedAudience")
            .as_::<ProtectedAudienceUtilities>()
    }
}
