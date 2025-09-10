use super::*;

/// The SVGSymbolElement class.
/// [`SVGSymbolElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGSymbolElement {
    inner: SVGGraphicsElement,
}

impl FromVal for SVGSymbolElement {
    fn from_val(v: &Any) -> Self {
        SVGSymbolElement {
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

impl core::ops::Deref for SVGSymbolElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGSymbolElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGSymbolElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGSymbolElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGSymbolElement> for Any {
    fn from(s: SVGSymbolElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGSymbolElement> for Any {
    fn from(s: &SVGSymbolElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGSymbolElement);

impl SVGSymbolElement {
    /// Getter of the `viewBox` attribute.
    /// [`SVGSymbolElement.viewBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/viewBox)
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGSymbolElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGSymbolElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
