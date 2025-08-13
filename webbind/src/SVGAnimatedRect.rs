use super::*;




/// The SVGAnimatedRect class.
/// [`SVGAnimatedRect`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedRect {
    inner: Any,
}

impl FromVal for SVGAnimatedRect {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedRect { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimatedRect {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedRect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedRect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedRect {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGAnimatedRect> for Any {
    fn from(s: SVGAnimatedRect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedRect> for Any {
    fn from(s: &SVGAnimatedRect) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedRect);


impl SVGAnimatedRect {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedRect.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/baseVal)
    pub fn base_val(&self) -> DOMRect {
        self.inner.get("baseVal").as_::<DOMRect>()
    }

}
impl SVGAnimatedRect {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedRect.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/animVal)
    pub fn anim_val(&self) -> DOMRectReadOnly {
        self.inner.get("animVal").as_::<DOMRectReadOnly>()
    }

}
