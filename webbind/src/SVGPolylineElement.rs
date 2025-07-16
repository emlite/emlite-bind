use super::*;

/// The SVGPolylineElement class.
/// [`SVGPolylineElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPolylineElement {
    inner: SVGGeometryElement,
}
impl FromVal for SVGPolylineElement {
    fn from_val(v: &Any) -> Self {
        SVGPolylineElement {
            inner: SVGGeometryElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGPolylineElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPolylineElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGPolylineElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGPolylineElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGPolylineElement> for Any {
    fn from(s: SVGPolylineElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGPolylineElement> for Any {
    fn from(s: &SVGPolylineElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGPolylineElement);

impl SVGPolylineElement {
    /// Getter of the `points` attribute.
    /// [`SVGPolylineElement.points`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/points)
    pub fn points(&self) -> SVGPointList {
        self.inner.get("points").as_::<SVGPointList>()
    }
}
impl SVGPolylineElement {
    /// Getter of the `animatedPoints` attribute.
    /// [`SVGPolylineElement.animatedPoints`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/animatedPoints)
    pub fn animated_points(&self) -> SVGPointList {
        self.inner.get("animatedPoints").as_::<SVGPointList>()
    }
}
