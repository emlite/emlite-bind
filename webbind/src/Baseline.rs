use super::*;

/// The Baseline class.
/// [`Baseline`](https://developer.mozilla.org/en-US/docs/Web/API/Baseline)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Baseline {
    inner: Any,
}
impl FromVal for Baseline {
    fn from_val(v: &Any) -> Self {
        Baseline {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Baseline {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Baseline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Baseline {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Baseline {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Baseline> for Any {
    fn from(s: Baseline) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Baseline> for Any {
    fn from(s: &Baseline) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Baseline);

impl Baseline {
    /// Getter of the `name` attribute.
    /// [`Baseline.name`](https://developer.mozilla.org/en-US/docs/Web/API/Baseline/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl Baseline {
    /// Getter of the `value` attribute.
    /// [`Baseline.value`](https://developer.mozilla.org/en-US/docs/Web/API/Baseline/value)
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }
}
