use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGSwitchElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGSwitchElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGSwitchElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGSwitchElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGSwitchElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGSwitchElement> for emlite::Val {
    fn from(s: SVGSwitchElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
