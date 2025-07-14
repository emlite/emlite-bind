use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGAnimatedString {
    inner: emlite::Val,
}
impl FromVal for SVGAnimatedString {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimatedString {
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
impl core::ops::Deref for SVGAnimatedString {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAnimatedString> for emlite::Val {
    fn from(s: SVGAnimatedString) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimatedString {
    pub fn base_val(&self) -> jsbind::DOMString {
        self.inner.get("baseVal").as_::<jsbind::DOMString>()
    }

    pub fn set_base_val(&mut self, value: jsbind::DOMString) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedString {
    pub fn anim_val(&self) -> jsbind::DOMString {
        self.inner.get("animVal").as_::<jsbind::DOMString>()
    }
}
