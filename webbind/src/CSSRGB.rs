use super::*;

/// The CSSRGB class.
/// [`CSSRGB`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRGB {
    inner: CSSColorValue,
}

impl FromVal for CSSRGB {
    fn from_val(v: &Any) -> Self {
        CSSRGB {
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

impl core::ops::Deref for CSSRGB {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSRGB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSRGB {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSRGB {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSRGB> for Any {
    fn from(s: CSSRGB) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSRGB> for Any {
    fn from(s: &CSSRGB) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSRGB);

impl CSSRGB {
    /// Getter of the `r` attribute.
    /// [`CSSRGB.r`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/r)
    pub fn r(&self) -> Any {
        self.inner.get("r").as_::<Any>()
    }

    /// Setter of the `r` attribute.
    /// [`CSSRGB.r`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/r)
    pub fn set_r(&mut self, value: &Any) {
        self.inner.set("r", value);
    }
}
impl CSSRGB {
    /// Getter of the `g` attribute.
    /// [`CSSRGB.g`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/g)
    pub fn g(&self) -> Any {
        self.inner.get("g").as_::<Any>()
    }

    /// Setter of the `g` attribute.
    /// [`CSSRGB.g`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/g)
    pub fn set_g(&mut self, value: &Any) {
        self.inner.set("g", value);
    }
}
impl CSSRGB {
    /// Getter of the `b` attribute.
    /// [`CSSRGB.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/b)
    pub fn b(&self) -> Any {
        self.inner.get("b").as_::<Any>()
    }

    /// Setter of the `b` attribute.
    /// [`CSSRGB.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/b)
    pub fn set_b(&mut self, value: &Any) {
        self.inner.set("b", value);
    }
}
impl CSSRGB {
    /// Getter of the `alpha` attribute.
    /// [`CSSRGB.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSRGB.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRGB/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}

impl CSSRGB {
    /// The `new CSSRGB(..)` constructor, creating a new CSSRGB instance
    pub fn new(r: &Any, g: &Any, b: &Any) -> CSSRGB {
        Self {
            inner: Any::global("CSSRGB")
                .new(&[r.into(), g.into(), b.into()])
                .as_::<CSSColorValue>(),
        }
    }
}

impl CSSRGB {
    /// The `new CSSRGB(..)` constructor, creating a new CSSRGB instance
    pub fn new_with_alpha(r: &Any, g: &Any, b: &Any, alpha: &Any) -> CSSRGB {
        Self {
            inner: Any::global("CSSRGB")
                .new(&[r.into(), g.into(), b.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
