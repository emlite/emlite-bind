use super::*;

/// The SVGUseElement class.
/// [`SVGUseElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGUseElement {
    inner: SVGGraphicsElement,
}

impl FromVal for SVGUseElement {
    fn from_val(v: &Any) -> Self {
        SVGUseElement {
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

impl core::ops::Deref for SVGUseElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGUseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGUseElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGUseElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGUseElement> for Any {
    fn from(s: SVGUseElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGUseElement> for Any {
    fn from(s: &SVGUseElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGUseElement);

impl SVGUseElement {
    /// Getter of the `x` attribute.
    /// [`SVGUseElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/x)
    pub fn x(&self) -> SVGAnimatedLength {
        self.inner.get("x").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    /// Getter of the `y` attribute.
    /// [`SVGUseElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/y)
    pub fn y(&self) -> SVGAnimatedLength {
        self.inner.get("y").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    /// Getter of the `width` attribute.
    /// [`SVGUseElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/width)
    pub fn width(&self) -> SVGAnimatedLength {
        self.inner.get("width").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    /// Getter of the `height` attribute.
    /// [`SVGUseElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/height)
    pub fn height(&self) -> SVGAnimatedLength {
        self.inner.get("height").as_::<SVGAnimatedLength>()
    }
}
impl SVGUseElement {
    /// Getter of the `instanceRoot` attribute.
    /// [`SVGUseElement.instanceRoot`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/instanceRoot)
    pub fn instance_root(&self) -> SVGElement {
        self.inner.get("instanceRoot").as_::<SVGElement>()
    }
}
impl SVGUseElement {
    /// Getter of the `animatedInstanceRoot` attribute.
    /// [`SVGUseElement.animatedInstanceRoot`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/animatedInstanceRoot)
    pub fn animated_instance_root(&self) -> SVGElement {
        self.inner.get("animatedInstanceRoot").as_::<SVGElement>()
    }
}
impl SVGUseElement {
    /// Getter of the `href` attribute.
    /// [`SVGUseElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGUseElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
