use super::*;

#[derive(Clone, Debug)]
pub struct CSSLayerStatementRule {
    inner: CSSRule,
}
impl FromVal for CSSLayerStatementRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSLayerStatementRule {
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
impl std::ops::Deref for CSSLayerStatementRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSLayerStatementRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSLayerStatementRule> for emlite::Val {
    fn from(s: CSSLayerStatementRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSLayerStatementRule {
    pub fn name_list(&self) -> jsbind::FrozenArray<jsbind::CSSOMString> {
        self.inner
            .get("nameList")
            .as_::<jsbind::FrozenArray<jsbind::CSSOMString>>()
    }
}
