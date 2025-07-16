use super::*;

/// The SVGMPathElement class.
/// [`SVGMPathElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGMPathElement {
    inner: SVGElement,
}
impl FromVal for SVGMPathElement {
    fn from_val(v: &Any) -> Self {
        SVGMPathElement {
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
impl core::ops::Deref for SVGMPathElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGMPathElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGMPathElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGMPathElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGMPathElement> for Any {
    fn from(s: SVGMPathElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGMPathElement> for Any {
    fn from(s: &SVGMPathElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGMPathElement);

impl SVGMPathElement {
    /// Getter of the `href` attribute.
    /// [`SVGMPathElement.href`](https://developer.mozilla.org/en-US/docs/Web/API/SVGMPathElement/href)
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
