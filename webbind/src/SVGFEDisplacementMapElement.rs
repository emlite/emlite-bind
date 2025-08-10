use super::*;

/// The SVGFEDisplacementMapElement class.
/// [`SVGFEDisplacementMapElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGFEDisplacementMapElement {
    inner: SVGElement,
}

impl FromVal for SVGFEDisplacementMapElement {
    fn from_val(v: &Any) -> Self {
        SVGFEDisplacementMapElement {
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

impl core::ops::Deref for SVGFEDisplacementMapElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGFEDisplacementMapElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGFEDisplacementMapElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGFEDisplacementMapElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGFEDisplacementMapElement> for Any {
    fn from(s: SVGFEDisplacementMapElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGFEDisplacementMapElement> for Any {
    fn from(s: &SVGFEDisplacementMapElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGFEDisplacementMapElement);

impl SVGFEDisplacementMapElement {
    /// Getter of the `in1` attribute.
    /// [`SVGFEDisplacementMapElement.in1`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in1)
    pub fn in1(&self) -> SVGAnimatedString {
        self.inner.get("in1").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `in2` attribute.
    /// [`SVGFEDisplacementMapElement.in2`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in2)
    pub fn in2(&self) -> SVGAnimatedString {
        self.inner.get("in2").as_::<SVGAnimatedString>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `scale` attribute.
    /// [`SVGFEDisplacementMapElement.scale`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/scale)
    pub fn scale(&self) -> SVGAnimatedNumber {
        self.inner.get("scale").as_::<SVGAnimatedNumber>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `xChannelSelector` attribute.
    /// [`SVGFEDisplacementMapElement.xChannelSelector`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/xChannelSelector)
    pub fn x_channel_selector(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("xChannelSelector")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `yChannelSelector` attribute.
    /// [`SVGFEDisplacementMapElement.yChannelSelector`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/yChannelSelector)
    pub fn y_channel_selector(&self) -> SVGAnimatedEnumeration {
        self.inner
            .get("yChannelSelector")
            .as_::<SVGAnimatedEnumeration>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `x` attribute.
    /// [`SVGFEDisplacementMapElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `y` attribute.
    /// [`SVGFEDisplacementMapElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `width` attribute.
    /// [`SVGFEDisplacementMapElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `height` attribute.
    /// [`SVGFEDisplacementMapElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGFEDisplacementMapElement {
    /// Getter of the `result` attribute.
    /// [`SVGFEDisplacementMapElement.result`](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/result)
    pub fn result(&self) -> SVGAnimatedString {
        self.inner.get("result").as_::<SVGAnimatedString>()
    }
}
