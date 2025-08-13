use super::*;




/// The CSSLab class.
/// [`CSSLab`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSLab {
    inner: CSSColorValue,
}

impl FromVal for CSSLab {
    fn from_val(v: &Any) -> Self {
        CSSLab { inner: CSSColorValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSLab {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSLab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSLab {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSLab {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSLab> for Any {
    fn from(s: CSSLab) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSLab> for Any {
    fn from(s: &CSSLab) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSLab);



impl CSSLab {
    /// The `new CSSLab(..)` constructor, creating a new CSSLab instance
    pub fn new0(l: &Any, a: &Any, b: &Any) -> CSSLab {
        Self {
            inner: Any::global("CSSLab").new(&[l.into(), a.into(), b.into()]).as_::<CSSColorValue>(),
        }
    }

    /// The `new CSSLab(..)` constructor, creating a new CSSLab instance
    pub fn new1(l: &Any, a: &Any, b: &Any, alpha: &Any) -> CSSLab {
        Self {
            inner: Any::global("CSSLab").new(&[l.into(), a.into(), b.into(), alpha.into()]).as_::<CSSColorValue>(),
        }
    }

}
impl CSSLab {
    /// Getter of the `l` attribute.
    /// [`CSSLab.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/l)
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    /// Setter of the `l` attribute.
    /// [`CSSLab.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/l)
    pub fn set_l(&mut self, value: &Any) {
        self.inner.set("l", value);
    }
}
impl CSSLab {
    /// Getter of the `a` attribute.
    /// [`CSSLab.a`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/a)
    pub fn a(&self) -> Any {
        self.inner.get("a").as_::<Any>()
    }

    /// Setter of the `a` attribute.
    /// [`CSSLab.a`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/a)
    pub fn set_a(&mut self, value: &Any) {
        self.inner.set("a", value);
    }
}
impl CSSLab {
    /// Getter of the `b` attribute.
    /// [`CSSLab.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/b)
    pub fn b(&self) -> Any {
        self.inner.get("b").as_::<Any>()
    }

    /// Setter of the `b` attribute.
    /// [`CSSLab.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/b)
    pub fn set_b(&mut self, value: &Any) {
        self.inner.set("b", value);
    }
}
impl CSSLab {
    /// Getter of the `alpha` attribute.
    /// [`CSSLab.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSLab.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLab/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}
