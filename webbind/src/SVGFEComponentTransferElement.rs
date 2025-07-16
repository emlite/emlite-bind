use super::*;

/// The SVGFEComponentTransferElement class.
/// [`SVGFEComponentTransferElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEComponentTransferElement {
    inner: SVGElement,
}
impl FromVal for SVGFEComponentTransferElement {
    fn from_val(v: &Any) -> Self {
        SVGFEComponentTransferElement {
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
impl core::ops::Deref for SVGFEComponentTransferElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGFEComponentTransferElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGFEComponentTransferElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGFEComponentTransferElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGFEComponentTransferElement> for Any {
    fn from(s: SVGFEComponentTransferElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGFEComponentTransferElement> for Any {
    fn from(s: &SVGFEComponentTransferElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGFEComponentTransferElement);

impl SVGFEComponentTransferElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEComponentTransferElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEComponentTransferElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEComponentTransferElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEComponentTransferElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEComponentTransferElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEComponentTransferElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEComponentTransferElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEComponentTransferElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEComponentTransferElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEComponentTransferElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEComponentTransferElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
