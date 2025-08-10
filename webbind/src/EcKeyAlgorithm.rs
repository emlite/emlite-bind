use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EcKeyAlgorithm {
    inner: Any,
}
impl FromVal for EcKeyAlgorithm {
    fn from_val(v: &Any) -> Self {
        EcKeyAlgorithm { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for EcKeyAlgorithm {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EcKeyAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EcKeyAlgorithm {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EcKeyAlgorithm {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EcKeyAlgorithm> for Any {
    fn from(s: EcKeyAlgorithm) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EcKeyAlgorithm> for Any {
    fn from(s: &EcKeyAlgorithm) -> Any {
        s.inner.clone()
    }
}

impl EcKeyAlgorithm {
    pub fn named_curve(&self) -> Any {
        self.inner.get("namedCurve").as_::<Any>()
    }

    pub fn set_named_curve(&mut self, value: &Any) {
        self.inner.set("namedCurve", value);
    }
}
