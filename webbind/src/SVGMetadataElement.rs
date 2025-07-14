use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGMetadataElement {
    inner: SVGElement,
}
impl FromVal for SVGMetadataElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGMetadataElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGMetadataElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGMetadataElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGMetadataElement> for emlite::Val {
    fn from(s: SVGMetadataElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
