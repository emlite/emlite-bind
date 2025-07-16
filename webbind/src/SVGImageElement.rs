use super::*;

/// The SVGImageElement class.
/// [`SVGImageElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGImageElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGImageElement {
    fn from_val(v: &Any) -> Self {
        SVGImageElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGImageElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGImageElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGImageElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGImageElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGImageElement> for Any {
    fn from(s: SVGImageElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGImageElement> for Any {
    fn from(s: &SVGImageElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGImageElement);

impl SVGImageElement {
    /// Getter of the `x` attribute.
    /// [`SVGImageElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGImageElement {
    /// Getter of the `y` attribute.
    /// [`SVGImageElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGImageElement {
    /// Getter of the `width` attribute.
    /// [`SVGImageElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGImageElement {
    /// Getter of the `height` attribute.
    /// [`SVGImageElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGImageElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGImageElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
impl SVGImageElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`SVGImageElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/crossOrigin)
    pub fn cross_origin(&self) -> String {
        self.inner.get("crossOrigin").as_::<String>()
    }

    /// Setter of the `crossOrigin` attribute.
    /// [`SVGImageElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/crossOrigin)
    pub fn set_cross_origin(&mut self, value: &str) {
        self.inner.set("crossOrigin", value);
    }
}
impl SVGImageElement {
    /// Getter of the `href` attribute.
    /// [`SVGImageElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
