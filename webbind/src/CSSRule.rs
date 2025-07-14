use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRule {
    inner: emlite::Val,
}
impl FromVal for CSSRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSRule {
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
impl core::ops::Deref for CSSRule {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSRule> for emlite::Val {
    fn from(s: CSSRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSRule);

impl CSSRule {
    pub fn css_text(&self) -> jsbind::CSSOMString {
        self.inner.get("cssText").as_::<jsbind::CSSOMString>()
    }

    pub fn set_css_text(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("cssText", value);
    }
}
impl CSSRule {
    pub fn parent_rule(&self) -> CSSRule {
        self.inner.get("parentRule").as_::<CSSRule>()
    }
}
impl CSSRule {
    pub fn parent_style_sheet(&self) -> CSSStyleSheet {
        self.inner.get("parentStyleSheet").as_::<CSSStyleSheet>()
    }
}
impl CSSRule {
    pub fn type_(&self) -> u16 {
        self.inner.get("type").as_::<u16>()
    }
}
