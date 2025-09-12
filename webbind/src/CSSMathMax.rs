use super::*;

/// The CSSMathMax class.
/// [`CSSMathMax`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathMax)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathMax {
    inner: CSSMathValue,
}

impl FromVal for CSSMathMax {
    fn from_val(v: &Any) -> Self {
        CSSMathMax {
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

impl core::ops::Deref for CSSMathMax {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathMax {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathMax {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathMax {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSMathMax> for Any {
    fn from(s: CSSMathMax) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathMax> for Any {
    fn from(s: &CSSMathMax) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathMax);

impl CSSMathMax {
    /// Getter of the `values` attribute.
    /// [`CSSMathMax.values`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathMax/values)
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}

impl CSSMathMax {
    /// The `new CSSMathMax(..)` constructor, creating a new CSSMathMax instance
    pub fn new(args: &Any) -> CSSMathMax {
        Self {
            inner: Any::global("CSSMathMax")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
