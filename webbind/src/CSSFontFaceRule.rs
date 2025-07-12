use super::*;

#[derive(Clone, Debug)]
pub struct CSSFontFaceRule {
    inner: CSSRule,
}
impl FromVal for CSSFontFaceRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFontFaceRule {
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
impl std::ops::Deref for CSSFontFaceRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSFontFaceRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSFontFaceRule> for emlite::Val {
    fn from(s: CSSFontFaceRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSFontFaceRule {
    pub fn style(&self) -> CSSFontFaceDescriptors {
        self.inner.get("style").as_::<CSSFontFaceDescriptors>()
    }
}
