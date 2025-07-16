use super::*;

/// The SVGAnimatedPreserveAspectRatio class.
/// [`SVGAnimatedPreserveAspectRatio`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedPreserveAspectRatio {
    inner: Any,
}
impl FromVal for SVGAnimatedPreserveAspectRatio {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedPreserveAspectRatio {
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
impl core::ops::Deref for SVGAnimatedPreserveAspectRatio {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedPreserveAspectRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAnimatedPreserveAspectRatio {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAnimatedPreserveAspectRatio {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAnimatedPreserveAspectRatio> for Any {
    fn from(s: SVGAnimatedPreserveAspectRatio) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAnimatedPreserveAspectRatio> for Any {
    fn from(s: &SVGAnimatedPreserveAspectRatio) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedPreserveAspectRatio);

impl SVGAnimatedPreserveAspectRatio {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedPreserveAspectRatio.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/baseVal)
    pub fn base_val(&self) -> SVGPreserveAspectRatio {
        self.inner.get("baseVal").as_::<SVGPreserveAspectRatio>()
    }
}
impl SVGAnimatedPreserveAspectRatio {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedPreserveAspectRatio.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/animVal)
    pub fn anim_val(&self) -> SVGPreserveAspectRatio {
        self.inner.get("animVal").as_::<SVGPreserveAspectRatio>()
    }
}
