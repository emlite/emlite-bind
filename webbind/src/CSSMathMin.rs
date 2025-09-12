use super::*;

/// The CSSMathMin class.
/// [`CSSMathMin`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathMin)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathMin {
    inner: CSSMathValue,
}

impl FromVal for CSSMathMin {
    fn from_val(v: &Any) -> Self {
        CSSMathMin {
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

impl core::ops::Deref for CSSMathMin {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathMin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathMin {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathMin {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSMathMin> for Any {
    fn from(s: CSSMathMin) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathMin> for Any {
    fn from(s: &CSSMathMin) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathMin);

impl CSSMathMin {
    /// Getter of the `values` attribute.
    /// [`CSSMathMin.values`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathMin/values)
    pub fn values(&self) -> CSSNumericArray {
        self.inner.get("values").as_::<CSSNumericArray>()
    }
}

impl CSSMathMin {
    /// The `new CSSMathMin(..)` constructor, creating a new CSSMathMin instance
    pub fn new(args: &Any) -> CSSMathMin {
        Self {
            inner: Any::global("CSSMathMin")
                .new(&[args.into()])
                .as_::<CSSMathValue>(),
        }
    }
}
