use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HmacKeyAlgorithm {
    inner: Any,
}
impl FromVal for HmacKeyAlgorithm {
    fn from_val(v: &Any) -> Self {
        HmacKeyAlgorithm { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HmacKeyAlgorithm {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HmacKeyAlgorithm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HmacKeyAlgorithm {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HmacKeyAlgorithm {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HmacKeyAlgorithm> for Any {
    fn from(s: HmacKeyAlgorithm) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HmacKeyAlgorithm> for Any {
    fn from(s: &HmacKeyAlgorithm) -> Any {
        s.inner.clone()
    }
}

impl HmacKeyAlgorithm {
    pub fn hash(&self) -> KeyAlgorithm {
        self.inner.get("hash").as_::<KeyAlgorithm>()
    }

    pub fn set_hash(&mut self, value: &KeyAlgorithm) {
        self.inner.set("hash", value);
    }
}
impl HmacKeyAlgorithm {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
