use super::*;

/// The SVGAnimatedLength class.
/// [`SVGAnimatedLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedLength {
    inner: Any,
}
impl FromVal for SVGAnimatedLength {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedLength {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGAnimatedLength {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAnimatedLength {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAnimatedLength {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAnimatedLength> for Any {
    fn from(s: SVGAnimatedLength) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAnimatedLength> for Any {
    fn from(s: &SVGAnimatedLength) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedLength);

impl SVGAnimatedLength {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedLength.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/baseVal)
    pub fn base_val(&self) -> SVGLength {
        self.inner.get("baseVal").as_::<SVGLength>()
    }
}
impl SVGAnimatedLength {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedLength.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/animVal)
    pub fn anim_val(&self) -> SVGLength {
        self.inner.get("animVal").as_::<SVGLength>()
    }
}
