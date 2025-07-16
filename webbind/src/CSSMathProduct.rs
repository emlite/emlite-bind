use super::*;

/// The CSSMathProduct class.
/// [`CSSMathProduct`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathProduct)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathProduct {
    inner: CSSMathValue,
}
impl FromVal for CSSMathProduct {
    fn from_val(v: &Any) -> Self {
        CSSMathProduct {
            inner: CSSMathValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMathProduct {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMathProduct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSMathProduct {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSMathProduct {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSMathProduct> for Any {
    fn from(s: CSSMathProduct) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSMathProduct> for Any {
    fn from(s: &CSSMathProduct) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSMathProduct);

impl CSSMathProduct {
    /// The `new CSSMathProduct(..)` constructor, creating a new CSSMathProduct instance
    pub fn new(args: &Any) -> CSSMathProduct {
        Self {
            inner: Any::global("CSSMathProduct")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
impl CSSMathProduct {
    /// Getter of the `values` attribute.
    /// [`CSSMathProduct.values`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathProduct/values)
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}
