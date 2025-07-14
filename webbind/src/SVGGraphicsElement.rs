use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGBoundingBoxOptions {
    inner: emlite::Val,
}
impl FromVal for SVGBoundingBoxOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SVGBoundingBoxOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGBoundingBoxOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGBoundingBoxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGBoundingBoxOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGBoundingBoxOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGBoundingBoxOptions> for emlite::Val {
    fn from(s: SVGBoundingBoxOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGBoundingBoxOptions {
    pub fn fill(&self) -> bool {
        self.inner.get("fill").as_::<bool>()
    }

    pub fn set_fill(&mut self, value: bool) {
        self.inner.set("fill", value);
    }
}
impl SVGBoundingBoxOptions {
    pub fn stroke(&self) -> bool {
        self.inner.get("stroke").as_::<bool>()
    }

    pub fn set_stroke(&mut self, value: bool) {
        self.inner.set("stroke", value);
    }
}
impl SVGBoundingBoxOptions {
    pub fn markers(&self) -> bool {
        self.inner.get("markers").as_::<bool>()
    }

    pub fn set_markers(&mut self, value: bool) {
        self.inner.set("markers", value);
    }
}
impl SVGBoundingBoxOptions {
    pub fn clipped(&self) -> bool {
        self.inner.get("clipped").as_::<bool>()
    }

    pub fn set_clipped(&mut self, value: bool) {
        self.inner.set("clipped", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGGraphicsElement {
    inner: SVGElement,
}
impl FromVal for SVGGraphicsElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGGraphicsElement {
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
impl core::ops::Deref for SVGGraphicsElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGGraphicsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGGraphicsElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGGraphicsElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGGraphicsElement> for emlite::Val {
    fn from(s: SVGGraphicsElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGGraphicsElement);

impl SVGGraphicsElement {
    pub fn transform(&self) -> SVGAnimatedTransformList {
        self.inner
            .get("transform")
            .as_::<SVGAnimatedTransformList>()
    }
}
impl SVGGraphicsElement {
    pub fn get_b_box0(&self) -> DOMRect {
        self.inner.call("getBBox", &[]).as_::<DOMRect>()
    }

    pub fn get_b_box1(&self, options: SVGBoundingBoxOptions) -> DOMRect {
        self.inner
            .call("getBBox", &[options.into()])
            .as_::<DOMRect>()
    }
}
impl SVGGraphicsElement {
    pub fn get_ctm(&self) -> DOMMatrix {
        self.inner.call("getCTM", &[]).as_::<DOMMatrix>()
    }
}
impl SVGGraphicsElement {
    pub fn get_screen_ctm(&self) -> DOMMatrix {
        self.inner.call("getScreenCTM", &[]).as_::<DOMMatrix>()
    }
}
impl SVGGraphicsElement {
    pub fn required_extensions(&self) -> SVGStringList {
        self.inner.get("requiredExtensions").as_::<SVGStringList>()
    }
}
impl SVGGraphicsElement {
    pub fn system_language(&self) -> SVGStringList {
        self.inner.get("systemLanguage").as_::<SVGStringList>()
    }
}
