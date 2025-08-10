use super::*;

/// The SVGViewElement class.
/// [`SVGViewElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGViewElement {
    inner: SVGElement,
}

impl FromVal for SVGViewElement {
    fn from_val(v: &Any) -> Self {
        SVGViewElement {
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

impl core::ops::Deref for SVGViewElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGViewElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGViewElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGViewElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGViewElement> for Any {
    fn from(s: SVGViewElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGViewElement> for Any {
    fn from(s: &SVGViewElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGViewElement);

impl SVGViewElement {
    /// Getter of the `viewBox` attribute.
    /// [`SVGViewElement.viewBox`](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/viewBox)
    pub fn view_box(&self) -> SVGAnimatedRect {
        self.inner.get("viewBox").as_::<SVGAnimatedRect>()
    }
}
impl SVGViewElement {
    /// Getter of the `preserveAspectRatio` attribute.
    /// [`SVGViewElement.preserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/preserveAspectRatio)
    pub fn preserve_aspect_ratio(&self) -> SVGAnimatedPreserveAspectRatio {
        self.inner
            .get("preserveAspectRatio")
            .as_::<SVGAnimatedPreserveAspectRatio>()
    }
}
