use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPropertyRule {
    inner: CSSRule,
}
impl FromVal for CSSPropertyRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPropertyRule {
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
impl core::ops::Deref for CSSPropertyRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPropertyRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSPropertyRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSPropertyRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSPropertyRule> for emlite::Val {
    fn from(s: CSSPropertyRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSPropertyRule> for emlite::Val {
    fn from(s: &CSSPropertyRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPropertyRule);

impl CSSPropertyRule {
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }
}
impl CSSPropertyRule {
    pub fn syntax(&self) -> CSSOMString {
        self.inner.get("syntax").as_::<CSSOMString>()
    }
}
impl CSSPropertyRule {
    pub fn inherits(&self) -> bool {
        self.inner.get("inherits").as_::<bool>()
    }
}
impl CSSPropertyRule {
    pub fn initial_value(&self) -> CSSOMString {
        self.inner.get("initialValue").as_::<CSSOMString>()
    }
}
