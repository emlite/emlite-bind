use super::*;




/// The SVGAnimatedEnumeration class.
/// [`SVGAnimatedEnumeration`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedEnumeration {
    inner: Any,
}

impl FromVal for SVGAnimatedEnumeration {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedEnumeration { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimatedEnumeration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedEnumeration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedEnumeration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedEnumeration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGAnimatedEnumeration> for Any {
    fn from(s: SVGAnimatedEnumeration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedEnumeration> for Any {
    fn from(s: &SVGAnimatedEnumeration) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedEnumeration);


impl SVGAnimatedEnumeration {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedEnumeration.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)
    pub fn base_val(&self) -> u16 {
        self.inner.get("baseVal").as_::<u16>()
    }

    /// Setter of the `baseVal` attribute.
    /// [`SVGAnimatedEnumeration.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)
    pub fn set_base_val(&mut self, value: u16) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedEnumeration {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedEnumeration.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/animVal)
    pub fn anim_val(&self) -> u16 {
        self.inner.get("animVal").as_::<u16>()
    }

}
