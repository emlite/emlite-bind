use super::*;




/// The SVGAnimatedNumber class.
/// [`SVGAnimatedNumber`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumber)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedNumber {
    inner: Any,
}

impl FromVal for SVGAnimatedNumber {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedNumber { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimatedNumber {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedNumber {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedNumber {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGAnimatedNumber> for Any {
    fn from(s: SVGAnimatedNumber) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedNumber> for Any {
    fn from(s: &SVGAnimatedNumber) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedNumber);


impl SVGAnimatedNumber {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedNumber.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumber/baseVal)
    pub fn base_val(&self) -> f32 {
        self.inner.get("baseVal").as_::<f32>()
    }

    /// Setter of the `baseVal` attribute.
    /// [`SVGAnimatedNumber.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumber/baseVal)
    pub fn set_base_val(&mut self, value: f32) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedNumber {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedNumber.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumber/animVal)
    pub fn anim_val(&self) -> f32 {
        self.inner.get("animVal").as_::<f32>()
    }

}
