use super::*;

/// The SVGAnimatedNumberList class.
/// [`SVGAnimatedNumberList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedNumberList {
    inner: Any,
}
impl FromVal for SVGAnimatedNumberList {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedNumberList {
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
impl core::ops::Deref for SVGAnimatedNumberList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedNumberList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAnimatedNumberList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAnimatedNumberList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAnimatedNumberList> for Any {
    fn from(s: SVGAnimatedNumberList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAnimatedNumberList> for Any {
    fn from(s: &SVGAnimatedNumberList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedNumberList);

impl SVGAnimatedNumberList {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedNumberList.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/baseVal)
    pub fn base_val(&self) -> SVGNumberList {
        self.inner.get("baseVal").as_::<SVGNumberList>()
    }
}
impl SVGAnimatedNumberList {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedNumberList.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/animVal)
    pub fn anim_val(&self) -> SVGNumberList {
        self.inner.get("animVal").as_::<SVGNumberList>()
    }
}
