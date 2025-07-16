use super::*;

/// The SVGNumber class.
/// [`SVGNumber`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGNumber {
    inner: Any,
}
impl FromVal for SVGNumber {
    fn from_val(v: &Any) -> Self {
        SVGNumber {
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
impl core::ops::Deref for SVGNumber {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGNumber {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGNumber {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGNumber> for Any {
    fn from(s: SVGNumber) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGNumber> for Any {
    fn from(s: &SVGNumber) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGNumber);

impl SVGNumber {
    /// Getter of the `value` attribute.
    /// [`SVGNumber.value`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    /// Setter of the `value` attribute.
    /// [`SVGNumber.value`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)
    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
