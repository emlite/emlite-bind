use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGUseElementShadowRoot {
    inner: ShadowRoot,
}
impl FromVal for SVGUseElementShadowRoot {
    fn from_val(v: &emlite::Val) -> Self {
        SVGUseElementShadowRoot {
            inner: ShadowRoot::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGUseElementShadowRoot {
    type Target = ShadowRoot;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGUseElementShadowRoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGUseElementShadowRoot> for emlite::Val {
    fn from(s: SVGUseElementShadowRoot) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
