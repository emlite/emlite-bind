use super::*;

#[derive(Clone, Debug)]
pub struct SVGFEMergeNodeElement {
    inner: SVGElement,
}
impl FromVal for SVGFEMergeNodeElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEMergeNodeElement {
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
impl std::ops::Deref for SVGFEMergeNodeElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGFEMergeNodeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGFEMergeNodeElement> for emlite::Val {
    fn from(s: SVGFEMergeNodeElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGFEMergeNodeElement {
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
