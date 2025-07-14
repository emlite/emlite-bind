use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEPointLightElement {
    inner: SVGElement,
}
impl FromVal for SVGFEPointLightElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGFEPointLightElement {
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
impl core::ops::Deref for SVGFEPointLightElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEPointLightElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGFEPointLightElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGFEPointLightElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGFEPointLightElement> for emlite::Val {
    fn from(s: SVGFEPointLightElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEPointLightElement);

impl SVGFEPointLightElement {
    pub fn x(&self) -> SVGAnimatedNumber {
        self.inner.get("x").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEPointLightElement {
    pub fn y(&self) -> SVGAnimatedNumber {
        self.inner.get("y").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEPointLightElement {
    pub fn z(&self) -> SVGAnimatedNumber {
        self.inner.get("z").as_::<SVGAnimatedNumber>()
    }
}
