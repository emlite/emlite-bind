use super::*;




/// The SVGAnimatedInteger class.
/// [`SVGAnimatedInteger`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedInteger {
    inner: Any,
}

impl FromVal for SVGAnimatedInteger {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedInteger { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimatedInteger {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedInteger {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedInteger {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGAnimatedInteger> for Any {
    fn from(s: SVGAnimatedInteger) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedInteger> for Any {
    fn from(s: &SVGAnimatedInteger) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedInteger);


impl SVGAnimatedInteger {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedInteger.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)
    pub fn base_val(&self) -> i32 {
        self.inner.get("baseVal").as_::<i32>()
    }

    /// Setter of the `baseVal` attribute.
    /// [`SVGAnimatedInteger.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)
    pub fn set_base_val(&mut self, value: i32) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedInteger {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedInteger.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/animVal)
    pub fn anim_val(&self) -> i32 {
        self.inner.get("animVal").as_::<i32>()
    }

}
