use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaKeyAlgorithm {
    inner: Any,
}
impl FromVal for RsaKeyAlgorithm {
    fn from_val(v: &Any) -> Self {
        RsaKeyAlgorithm { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RsaKeyAlgorithm {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RsaKeyAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RsaKeyAlgorithm {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RsaKeyAlgorithm {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RsaKeyAlgorithm> for Any {
    fn from(s: RsaKeyAlgorithm) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RsaKeyAlgorithm> for Any {
    fn from(s: &RsaKeyAlgorithm) -> Any {
        s.inner.clone()
    }
}

impl RsaKeyAlgorithm {
    pub fn modulus_length(&self) -> u32 {
        self.inner.get("modulusLength").as_::<u32>()
    }

    pub fn set_modulus_length(&mut self, value: u32) {
        self.inner.set("modulusLength", value);
    }
}
impl RsaKeyAlgorithm {
    pub fn public_exponent(&self) -> Any {
        self.inner.get("publicExponent").as_::<Any>()
    }

    pub fn set_public_exponent(&mut self, value: &Any) {
        self.inner.set("publicExponent", value);
    }
}
