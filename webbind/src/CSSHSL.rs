use super::*;




/// The CSSHSL class.
/// [`CSSHSL`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSHSL {
    inner: CSSColorValue,
}

impl FromVal for CSSHSL {
    fn from_val(v: &Any) -> Self {
        CSSHSL { inner: CSSColorValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSHSL {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSHSL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSHSL {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSHSL {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSHSL> for Any {
    fn from(s: CSSHSL) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSHSL> for Any {
    fn from(s: &CSSHSL) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSHSL);



impl CSSHSL {
    /// The `new CSSHSL(..)` constructor, creating a new CSSHSL instance
    pub fn new0(h: &Any, s: &Any, l: &Any) -> CSSHSL {
        Self {
            inner: Any::global("CSSHSL").new(&[h.into(), s.into(), l.into()]).as_::<CSSColorValue>(),
        }
    }

    /// The `new CSSHSL(..)` constructor, creating a new CSSHSL instance
    pub fn new1(h: &Any, s: &Any, l: &Any, alpha: &Any) -> CSSHSL {
        Self {
            inner: Any::global("CSSHSL").new(&[h.into(), s.into(), l.into(), alpha.into()]).as_::<CSSColorValue>(),
        }
    }

}
impl CSSHSL {
    /// Getter of the `h` attribute.
    /// [`CSSHSL.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/h)
    pub fn h(&self) -> Any {
        self.inner.get("h").as_::<Any>()
    }

    /// Setter of the `h` attribute.
    /// [`CSSHSL.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/h)
    pub fn set_h(&mut self, value: &Any) {
        self.inner.set("h", value);
    }
}
impl CSSHSL {
    /// Getter of the `s` attribute.
    /// [`CSSHSL.s`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/s)
    pub fn s(&self) -> Any {
        self.inner.get("s").as_::<Any>()
    }

    /// Setter of the `s` attribute.
    /// [`CSSHSL.s`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/s)
    pub fn set_s(&mut self, value: &Any) {
        self.inner.set("s", value);
    }
}
impl CSSHSL {
    /// Getter of the `l` attribute.
    /// [`CSSHSL.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/l)
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    /// Setter of the `l` attribute.
    /// [`CSSHSL.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/l)
    pub fn set_l(&mut self, value: &Any) {
        self.inner.set("l", value);
    }
}
impl CSSHSL {
    /// Getter of the `alpha` attribute.
    /// [`CSSHSL.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSHSL.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSHSL/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}
