use super::*;

/// The CSSHWB class.
/// [`CSSHWB`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSHWB {
    inner: CSSColorValue,
}

impl FromVal for CSSHWB {
    fn from_val(v: &Any) -> Self {
        CSSHWB {
            inner: CSSColorValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSHWB {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSHWB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSHWB {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSHWB {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSHWB> for Any {
    fn from(s: CSSHWB) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSHWB> for Any {
    fn from(s: &CSSHWB) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSHWB);

impl CSSHWB {
    /// Getter of the `h` attribute.
    /// [`CSSHWB.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/h)
    pub fn h(&self) -> CSSNumericValue {
        self.inner.get("h").as_::<CSSNumericValue>()
    }

    /// Setter of the `h` attribute.
    /// [`CSSHWB.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/h)
    pub fn set_h(&mut self, value: &CSSNumericValue) {
        self.inner.set("h", value);
    }
}
impl CSSHWB {
    /// Getter of the `w` attribute.
    /// [`CSSHWB.w`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/w)
    pub fn w(&self) -> Any {
        self.inner.get("w").as_::<Any>()
    }

    /// Setter of the `w` attribute.
    /// [`CSSHWB.w`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/w)
    pub fn set_w(&mut self, value: &Any) {
        self.inner.set("w", value);
    }
}
impl CSSHWB {
    /// Getter of the `b` attribute.
    /// [`CSSHWB.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/b)
    pub fn b(&self) -> Any {
        self.inner.get("b").as_::<Any>()
    }

    /// Setter of the `b` attribute.
    /// [`CSSHWB.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/b)
    pub fn set_b(&mut self, value: &Any) {
        self.inner.set("b", value);
    }
}
impl CSSHWB {
    /// Getter of the `alpha` attribute.
    /// [`CSSHWB.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSHWB.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHWB/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}

impl CSSHWB {
    /// The `new CSSHWB(..)` constructor, creating a new CSSHWB instance
    pub fn new(h: &CSSNumericValue, w: &Any, b: &Any) -> CSSHWB {
        Self {
            inner: Any::global("CSSHWB")
                .new(&[h.into(), w.into(), b.into()])
                .as_::<CSSColorValue>(),
        }
    }
}

impl CSSHWB {
    /// The `new CSSHWB(..)` constructor, creating a new CSSHWB instance
    pub fn new_with_alpha(h: &CSSNumericValue, w: &Any, b: &Any, alpha: &Any) -> CSSHWB {
        Self {
            inner: Any::global("CSSHWB")
                .new(&[h.into(), w.into(), b.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
