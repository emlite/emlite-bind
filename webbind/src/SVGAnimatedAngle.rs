use super::*;

/// The SVGAnimatedAngle class.
/// [`SVGAnimatedAngle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedAngle {
    inner: Any,
}
impl FromVal for SVGAnimatedAngle {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedAngle {
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
impl core::ops::Deref for SVGAnimatedAngle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedAngle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAnimatedAngle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAnimatedAngle {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAnimatedAngle> for Any {
    fn from(s: SVGAnimatedAngle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAnimatedAngle> for Any {
    fn from(s: &SVGAnimatedAngle) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedAngle);

impl SVGAnimatedAngle {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedAngle.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/baseVal)
    pub fn base_val(&self) -> SVGAngle {
        self.inner.get("baseVal").as_::<SVGAngle>()
    }
}
impl SVGAnimatedAngle {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedAngle.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/animVal)
    pub fn anim_val(&self) -> SVGAngle {
        self.inner.get("animVal").as_::<SVGAngle>()
    }
}
