use super::*;

/// The SVGGradientElement class.
/// [`SVGGradientElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGGradientElement {
    inner: SVGElement,
}
impl FromVal for SVGGradientElement {
    fn from_val(v: &Any) -> Self {
        SVGGradientElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGGradientElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGGradientElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGGradientElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGGradientElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGGradientElement> for Any {
    fn from(s: SVGGradientElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGGradientElement> for Any {
    fn from(s: &SVGGradientElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGGradientElement);

impl SVGGradientElement {
    /// Getter of the `gradientUnits` attribute.
    /// [`SVGGradientElement.gradientUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientUnits)
    pub fn gradient_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("gradientUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGGradientElement {
    /// Getter of the `gradientTransform` attribute.
    /// [`SVGGradientElement.gradientTransform`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientTransform)
    pub fn gradient_transform(&self) -> SVGAnimatedTransformList {
        self.inner
            .get("gradientTransform")
            .as_::<SVGAnimatedTransformList>()
    }
}
impl SVGGradientElement {
    /// Getter of the `spreadMethod` attribute.
    /// [`SVGGradientElement.spreadMethod`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/spreadMethod)
    pub fn spread_method(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("spreadMethod")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGGradientElement {
    /// Getter of the `href` attribute.
    /// [`SVGGradientElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
