use super::*;

/// The SVGAnimatedBoolean class.
/// [`SVGAnimatedBoolean`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedBoolean {
    inner: Any,
}

impl FromVal for SVGAnimatedBoolean {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedBoolean {
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

impl core::ops::Deref for SVGAnimatedBoolean {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedBoolean {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedBoolean {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedBoolean {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGAnimatedBoolean> for Any {
    fn from(s: SVGAnimatedBoolean) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedBoolean> for Any {
    fn from(s: &SVGAnimatedBoolean) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedBoolean);

impl SVGAnimatedBoolean {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedBoolean.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)
    pub fn base_val(&self) -> bool {
        self.inner.get("baseVal").as_::<bool>()
    }

    /// Setter of the `baseVal` attribute.
    /// [`SVGAnimatedBoolean.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)
    pub fn set_base_val(&mut self, value: bool) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedBoolean {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedBoolean.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/animVal)
    pub fn anim_val(&self) -> bool {
        self.inner.get("animVal").as_::<bool>()
    }
}
