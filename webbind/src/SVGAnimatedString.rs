use super::*;

/// The SVGAnimatedString class.
/// [`SVGAnimatedString`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimatedString {
    inner: Any,
}
impl FromVal for SVGAnimatedString {
    fn from_val(v: &Any) -> Self {
        SVGAnimatedString {
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
impl core::ops::Deref for SVGAnimatedString {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAnimatedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAnimatedString {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAnimatedString {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAnimatedString> for Any {
    fn from(s: SVGAnimatedString) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAnimatedString> for Any {
    fn from(s: &SVGAnimatedString) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAnimatedString);

impl SVGAnimatedString {
    /// Getter of the `baseVal` attribute.
    /// [`SVGAnimatedString.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)
    pub fn base_val(&self) -> DOMString {
        self.inner.get("baseVal").as_::<DOMString>()
    }

    /// Setter of the `baseVal` attribute.
    /// [`SVGAnimatedString.baseVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)
    pub fn set_base_val(&mut self, value: &DOMString) {
        self.inner.set("baseVal", value);
    }
}
impl SVGAnimatedString {
    /// Getter of the `animVal` attribute.
    /// [`SVGAnimatedString.animVal`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/animVal)
    pub fn anim_val(&self) -> DOMString {
        self.inner.get("animVal").as_::<DOMString>()
    }
}
