use super::*;




/// The SVGAnimatedLengthList class.
/// [`SVGAnimatedLengthList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedLengthList {
    inner: Any,
}

impl FromVal for SVGAnimatedLengthList {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedLengthList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimatedLengthList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedLengthList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedLengthList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedLengthList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGAnimatedLengthList> for Any {
    fn from(s: SVGAnimatedLengthList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedLengthList> for Any {
    fn from(s: &SVGAnimatedLengthList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedLengthList);


impl SVGAnimatedLengthList {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedLengthList.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/baseVal)
    pub fn base_val(&self) -> SVGLengthList {
        self.inner.get("baseVal").as_::<SVGLengthList>()
    }

}
impl SVGAnimatedLengthList {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedLengthList.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/animVal)
    pub fn anim_val(&self) -> SVGLengthList {
        self.inner.get("animVal").as_::<SVGLengthList>()
    }

}
