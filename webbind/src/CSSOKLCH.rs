use super::*;

/// The CSSOKLCH class.
/// [`CSSOKLCH`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSOKLCH {
    inner: CSSColorValue,
}

impl FromVal for CSSOKLCH {
    fn from_val(v: &Any) -> Self {
        CSSOKLCH {
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

impl core::ops::Deref for CSSOKLCH {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSOKLCH {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSOKLCH {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSOKLCH {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSOKLCH> for Any {
    fn from(s: CSSOKLCH) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSOKLCH> for Any {
    fn from(s: &CSSOKLCH) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSOKLCH);

impl CSSOKLCH {
    /// Getter of the `l` attribute.
    /// [`CSSOKLCH.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/l)
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    /// Setter of the `l` attribute.
    /// [`CSSOKLCH.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/l)
    pub fn set_l(&mut self, value: &Any) {
        self.inner.set("l", value);
    }
}
impl CSSOKLCH {
    /// Getter of the `c` attribute.
    /// [`CSSOKLCH.c`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/c)
    pub fn c(&self) -> Any {
        self.inner.get("c").as_::<Any>()
    }

    /// Setter of the `c` attribute.
    /// [`CSSOKLCH.c`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/c)
    pub fn set_c(&mut self, value: &Any) {
        self.inner.set("c", value);
    }
}
impl CSSOKLCH {
    /// Getter of the `h` attribute.
    /// [`CSSOKLCH.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/h)
    pub fn h(&self) -> Any {
        self.inner.get("h").as_::<Any>()
    }

    /// Setter of the `h` attribute.
    /// [`CSSOKLCH.h`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/h)
    pub fn set_h(&mut self, value: &Any) {
        self.inner.set("h", value);
    }
}
impl CSSOKLCH {
    /// Getter of the `alpha` attribute.
    /// [`CSSOKLCH.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSOKLCH.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLCH/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}

impl CSSOKLCH {
    /// The `new CSSOKLCH(..)` constructor, creating a new CSSOKLCH instance
    pub fn new0(l: &Any, c: &Any, h: &Any) -> CSSOKLCH {
        Self {
            inner: Any::global("CSSOKLCH")
                .new(&[l.into(), c.into(), h.into()])
                .as_::<CSSColorValue>(),
        }
    }

    /// The `new CSSOKLCH(..)` constructor, creating a new CSSOKLCH instance
    pub fn new1(l: &Any, c: &Any, h: &Any, alpha: &Any) -> CSSOKLCH {
        Self {
            inner: Any::global("CSSOKLCH")
                .new(&[l.into(), c.into(), h.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
