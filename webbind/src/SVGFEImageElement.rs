use super::*;

/// The SVGFEImageElement class.
/// [`SVGFEImageElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEImageElement {
    inner: SVGElement,
}
impl FromVal for SVGFEImageElement {
    fn from_val(v: &Any) -> Self {
        SVGFEImageElement {
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
impl core::ops::Deref for SVGFEImageElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEImageElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEImageElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEImageElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEImageElement> for Any {
    fn from(s: SVGFEImageElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEImageElement> for Any {
    fn from(s: &SVGFEImageElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEImageElement);

impl SVGFEImageElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGFEImageElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`SVGFEImageElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/crossOrigin)
    pub fn cross_origin(&self) -> SVGAnimatedString {
        self.inner.get("crossOrigin").as_::<SVGAnimatedString>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEImageElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEImageElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEImageElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEImageElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEImageElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
impl SVGFEImageElement {
    /// Getter of the `href` attribute.
    /// [`SVGFEImageElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEImageElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
