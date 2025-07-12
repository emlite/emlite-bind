use super::*;

#[derive(Clone, Debug)]
pub struct CSSNestedDeclarations {
    inner: CSSRule,
}
impl FromVal for CSSNestedDeclarations {
    fn from_val(v: &emlite::Val) -> Self {
        CSSNestedDeclarations {
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
impl std::ops::Deref for CSSNestedDeclarations {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSNestedDeclarations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSNestedDeclarations> for emlite::Val {
    fn from(s: CSSNestedDeclarations) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSNestedDeclarations {
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }
}
