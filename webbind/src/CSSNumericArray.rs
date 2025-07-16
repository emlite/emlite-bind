use super::*;

/// The CSSNumericArray class.
/// [`CSSNumericArray`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericArray)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSNumericArray {
    inner: Any,
}
impl FromVal for CSSNumericArray {
    fn from_val(v: &Any) -> Self {
        CSSNumericArray {
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
impl core::ops::Deref for CSSNumericArray {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSNumericArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSNumericArray {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSNumericArray {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSNumericArray> for Any {
    fn from(s: CSSNumericArray) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSNumericArray> for Any {
    fn from(s: &CSSNumericArray) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSNumericArray);

impl CSSNumericArray {
    /// Getter of the `length` attribute.
    /// [`CSSNumericArray.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericArray/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
