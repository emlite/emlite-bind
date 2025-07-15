use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSColorProfileRule {
    inner: CSSRule,
}
impl FromVal for CSSColorProfileRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSColorProfileRule {
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
impl core::ops::Deref for CSSColorProfileRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSColorProfileRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSColorProfileRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSColorProfileRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSColorProfileRule> for emlite::Val {
    fn from(s: CSSColorProfileRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSColorProfileRule);

impl CSSColorProfileRule {
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }
}
impl CSSColorProfileRule {
    pub fn src(&self) -> CSSOMString {
        self.inner.get("src").as_::<CSSOMString>()
    }
}
impl CSSColorProfileRule {
    pub fn rendering_intent(&self) -> CSSOMString {
        self.inner.get("renderingIntent").as_::<CSSOMString>()
    }
}
impl CSSColorProfileRule {
    pub fn components(&self) -> CSSOMString {
        self.inner.get("components").as_::<CSSOMString>()
    }
}
