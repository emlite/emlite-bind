use super::*;

/// The SVGAnimatedTransformList class.
/// [`SVGAnimatedTransformList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedTransformList {
    inner: Any,
}

impl FromVal for SVGAnimatedTransformList {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedTransformList {
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

impl core::ops::Deref for SVGAnimatedTransformList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimatedTransformList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimatedTransformList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimatedTransformList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGAnimatedTransformList> for Any {
    fn from(s: SVGAnimatedTransformList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimatedTransformList> for Any {
    fn from(s: &SVGAnimatedTransformList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimatedTransformList);

impl SVGAnimatedTransformList {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedTransformList.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/baseVal)
    pub fn base_val(&self) -> SVGTransformList {
        self.inner.get("baseVal").as_::<SVGTransformList>()
    }
}
impl SVGAnimatedTransformList {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedTransformList.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/animVal)
    pub fn anim_val(&self) -> SVGTransformList {
        self.inner.get("animVal").as_::<SVGTransformList>()
    }
}
