use super::*;




/// The SVGPolygonElement class.
/// [`SVGPolygonElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPolygonElement {
    inner: SVGGeometryElement,
}

impl FromVal for SVGPolygonElement {
    fn from_val(v: &Any) -> Self {
        SVGPolygonElement { inner: SVGGeometryElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGPolygonElement {
    type Target = SVGGeometryElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGPolygonElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGPolygonElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGPolygonElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGPolygonElement> for Any {
    fn from(s: SVGPolygonElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGPolygonElement> for Any {
    fn from(s: &SVGPolygonElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGPolygonElement);


impl SVGPolygonElement {
    /// Getter of the `points` attribute.
    /// [`SVGPolygonElement.points`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/points)
    pub fn points(&self) -> SVGPointList {
        self.inner.get("points").as_::<SVGPointList>()
    }

}
impl SVGPolygonElement {
    /// Getter of the `animatedPoints` attribute.
    /// [`SVGPolygonElement.animatedPoints`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/animatedPoints)
    pub fn animated_points(&self) -> SVGPointList {
        self.inner.get("animatedPoints").as_::<SVGPointList>()
    }

}
