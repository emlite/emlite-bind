use super::*;

/// The CSSLCH class.
/// [`CSSLCH`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSLCH {
    inner: CSSColorValue,
}

impl FromVal for CSSLCH {
    fn from_val(v: &Any) -> Self {
        CSSLCH {
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

impl core::ops::Deref for CSSLCH {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSLCH {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSLCH {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSLCH {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSLCH> for Any {
    fn from(s: CSSLCH) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSLCH> for Any {
    fn from(s: &CSSLCH) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSLCH);

impl CSSLCH {
    /// The `new CSSLCH(..)` constructor, creating a new CSSLCH instance
    pub fn new0(l: &Any, c: &Any, h: &Any) -> CSSLCH {
        Self {
            inner: Any::global("CSSLCH")
                .new(&[l.into(), c.into(), h.into()])
                .as_::<CSSColorValue>(),
        }
    }

    /// The `new CSSLCH(..)` constructor, creating a new CSSLCH instance
    pub fn new1(l: &Any, c: &Any, h: &Any, alpha: &Any) -> CSSLCH {
        Self {
            inner: Any::global("CSSLCH")
                .new(&[l.into(), c.into(), h.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSLCH {
    /// Getter of the `l` attribute.
    /// [`CSSLCH.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/l)
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    /// Setter of the `l` attribute.
    /// [`CSSLCH.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/l)
    pub fn set_l(&mut self, value: &Any) {
        self.inner.set("l", value);
    }
}
impl CSSLCH {
    /// Getter of the `c` attribute.
    /// [`CSSLCH.c`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/c)
    pub fn c(&self) -> Any {
        self.inner.get("c").as_::<Any>()
    }

    /// Setter of the `c` attribute.
    /// [`CSSLCH.c`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/c)
    pub fn set_c(&mut self, value: &Any) {
        self.inner.set("c", value);
    }
}
impl CSSLCH {
    /// Getter of the `h` attribute.
    /// [`CSSLCH.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/h)
    pub fn h(&self) -> Any {
        self.inner.get("h").as_::<Any>()
    }

    /// Setter of the `h` attribute.
    /// [`CSSLCH.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/h)
    pub fn set_h(&mut self, value: &Any) {
        self.inner.set("h", value);
    }
}
impl CSSLCH {
    /// Getter of the `alpha` attribute.
    /// [`CSSLCH.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSLCH.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLCH/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}
