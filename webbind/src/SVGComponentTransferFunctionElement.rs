use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGComponentTransferFunctionElement {
    inner: SVGElement,
}
impl FromVal for SVGComponentTransferFunctionElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGComponentTransferFunctionElement {
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
impl core::ops::Deref for SVGComponentTransferFunctionElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGComponentTransferFunctionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGComponentTransferFunctionElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGComponentTransferFunctionElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGComponentTransferFunctionElement> for emlite::Val {
    fn from(s: SVGComponentTransferFunctionElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGComponentTransferFunctionElement> for emlite::Val {
    fn from(s: &SVGComponentTransferFunctionElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGComponentTransferFunctionElement);

impl SVGComponentTransferFunctionElement {
    pub fn type_(&self) -> SVGAnimatedEnumeration {
        self.inner.get("type").as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGComponentTransferFunctionElement {
    pub fn table_values(&self) -> SVGAnimatedNumberList {
        self.inner.get("tableValues").as_::<SVGAnimatedNumberList>()
    }
}
impl SVGComponentTransferFunctionElement {
    pub fn slope(&self) -> SVGAnimatedNumber {
        self.inner.get("slope").as_::<SVGAnimatedNumber>()
    }
}
impl SVGComponentTransferFunctionElement {
    pub fn intercept(&self) -> SVGAnimatedNumber {
        self.inner.get("intercept").as_::<SVGAnimatedNumber>()
    }
}
impl SVGComponentTransferFunctionElement {
    pub fn amplitude(&self) -> SVGAnimatedNumber {
        self.inner.get("amplitude").as_::<SVGAnimatedNumber>()
    }
}
impl SVGComponentTransferFunctionElement {
    pub fn exponent(&self) -> SVGAnimatedNumber {
        self.inner.get("exponent").as_::<SVGAnimatedNumber>()
    }
}
impl SVGComponentTransferFunctionElement {
    pub fn offset(&self) -> SVGAnimatedNumber {
        self.inner.get("offset").as_::<SVGAnimatedNumber>()
    }
}
