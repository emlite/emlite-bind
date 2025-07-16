use super::*;

/// The SVGFEMergeElement class.
/// [`SVGFEMergeElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEMergeElement {
    inner: SVGElement,
}
impl FromVal for SVGFEMergeElement {
    fn from_val(v: &Any) -> Self {
        SVGFEMergeElement {
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
impl core::ops::Deref for SVGFEMergeElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEMergeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEMergeElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEMergeElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEMergeElement> for Any {
    fn from(s: SVGFEMergeElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEMergeElement> for Any {
    fn from(s: &SVGFEMergeElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEMergeElement);

impl SVGFEMergeElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEMergeElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMergeElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEMergeElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMergeElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEMergeElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMergeElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEMergeElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEMergeElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEMergeElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
