use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPositionTryRule {
    inner: CSSRule,
}
impl FromVal for CSSPositionTryRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPositionTryRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSPositionTryRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPositionTryRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSPositionTryRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSPositionTryRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSPositionTryRule> for emlite::Val {
    fn from(s: CSSPositionTryRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSPositionTryRule> for emlite::Val {
    fn from(s: &CSSPositionTryRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPositionTryRule);

impl CSSPositionTryRule {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl CSSPositionTryRule {
    pub fn style(&self) -> CSSPositionTryDescriptors {
        self.inner.get("style").as_::<CSSPositionTryDescriptors>()
    }
}
