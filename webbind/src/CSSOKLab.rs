use super::*;

/// The CSSOKLab class.
/// [`CSSOKLab`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSOKLab {
    inner: CSSColorValue,
}

impl FromVal for CSSOKLab {
    fn from_val(v: &Any) -> Self {
        CSSOKLab {
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

impl core::ops::Deref for CSSOKLab {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSOKLab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSOKLab {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSOKLab {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSOKLab> for Any {
    fn from(s: CSSOKLab) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSOKLab> for Any {
    fn from(s: &CSSOKLab) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSOKLab);

impl CSSOKLab {
    /// Getter of the `l` attribute.
    /// [`CSSOKLab.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/l)
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    /// Setter of the `l` attribute.
    /// [`CSSOKLab.l`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/l)
    pub fn set_l(&mut self, value: &Any) {
        self.inner.set("l", value);
    }
}
impl CSSOKLab {
    /// Getter of the `a` attribute.
    /// [`CSSOKLab.a`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/a)
    pub fn a(&self) -> Any {
        self.inner.get("a").as_::<Any>()
    }

    /// Setter of the `a` attribute.
    /// [`CSSOKLab.a`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/a)
    pub fn set_a(&mut self, value: &Any) {
        self.inner.set("a", value);
    }
}
impl CSSOKLab {
    /// Getter of the `b` attribute.
    /// [`CSSOKLab.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/b)
    pub fn b(&self) -> Any {
        self.inner.get("b").as_::<Any>()
    }

    /// Setter of the `b` attribute.
    /// [`CSSOKLab.b`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/b)
    pub fn set_b(&mut self, value: &Any) {
        self.inner.set("b", value);
    }
}
impl CSSOKLab {
    /// Getter of the `alpha` attribute.
    /// [`CSSOKLab.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSOKLab.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSOKLab/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}

impl CSSOKLab {
    /// The `new CSSOKLab(..)` constructor, creating a new CSSOKLab instance
    pub fn new(l: &Any, a: &Any, b: &Any) -> CSSOKLab {
        Self {
            inner: Any::global("CSSOKLab")
                .new(&[l.into(), a.into(), b.into()])
                .as_::<CSSColorValue>(),
        }
    }
}

impl CSSOKLab {
    /// The `new CSSOKLab(..)` constructor, creating a new CSSOKLab instance
    pub fn new_with_alpha(l: &Any, a: &Any, b: &Any, alpha: &Any) -> CSSOKLab {
        Self {
            inner: Any::global("CSSOKLab")
                .new(&[l.into(), a.into(), b.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
