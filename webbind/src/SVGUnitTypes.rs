use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGUnitTypes {
    inner: emlite::Val,
}
impl FromVal for SVGUnitTypes {
    fn from_val(v: &emlite::Val) -> Self {
        SVGUnitTypes {
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
impl core::ops::Deref for SVGUnitTypes {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGUnitTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGUnitTypes> for emlite::Val {
    fn from(s: SVGUnitTypes) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
