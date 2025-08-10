use super::*;

/// The SVGFilterElement class.
/// [`SVGFilterElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFilterElement {
    inner: SVGElement,
}

impl FromVal for SVGFilterElement {
    fn from_val(v: &Any) -> Self {
        SVGFilterElement {
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

impl core::ops::Deref for SVGFilterElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFilterElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFilterElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFilterElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFilterElement> for Any {
    fn from(s: SVGFilterElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFilterElement> for Any {
    fn from(s: &SVGFilterElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFilterElement);

impl SVGFilterElement {
    /// Getter of the `filterUnits` attribute.
    /// [`SVGFilterElement.filterUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/filterUnits)
    pub fn filter_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("filterUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFilterElement {
    /// Getter of the `primitiveUnits` attribute.
    /// [`SVGFilterElement.primitiveUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/primitiveUnits)
    pub fn primitive_units(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("primitiveUnits")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFilterElement {
    /// Getter of the `x` attribute.
    /// [`SVGFilterElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    /// Getter of the `y` attribute.
    /// [`SVGFilterElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    /// Getter of the `width` attribute.
    /// [`SVGFilterElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    /// Getter of the `height` attribute.
    /// [`SVGFilterElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFilterElement {
    /// Getter of the `href` attribute.
    /// [`SVGFilterElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
