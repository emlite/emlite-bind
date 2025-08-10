use super::*;

/// The CSSMathSum class.
/// [`CSSMathSum`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathSum)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathSum {
    inner: CSSMathValue,
}

impl FromVal for CSSMathSum {
    fn from_val(v: &Any) -> Self {
        CSSMathSum {
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

impl core::ops::Deref for CSSMathSum {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathSum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathSum {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathSum {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSMathSum> for Any {
    fn from(s: CSSMathSum) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathSum> for Any {
    fn from(s: &CSSMathSum) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathSum);

impl CSSMathSum {
    /// The `new CSSMathSum(..)` constructor, creating a new CSSMathSum instance
    pub fn new(args: &Any) -> CSSMathSum {
        Self {
            inner: Any::global("CSSMathSum")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
impl CSSMathSum {
    /// Getter of the `values` attribute.
    /// [`CSSMathSum.values`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathSum/values)
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}
