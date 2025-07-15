use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for InterestGroupScriptRunnerGlobalScope {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InterestGroupScriptRunnerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for InterestGroupScriptRunnerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InterestGroupScriptRunnerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<InterestGroupScriptRunnerGlobalScope> for emlite::Val {
    fn from(s: InterestGroupScriptRunnerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(InterestGroupScriptRunnerGlobalScope);

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
