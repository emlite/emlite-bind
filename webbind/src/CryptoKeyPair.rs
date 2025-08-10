use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CryptoKeyPair {
    inner: Any,
}
impl FromVal for CryptoKeyPair {
    fn from_val(v: &Any) -> Self {
        CryptoKeyPair { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CryptoKeyPair {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CryptoKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CryptoKeyPair {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CryptoKeyPair {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CryptoKeyPair> for Any {
    fn from(s: CryptoKeyPair) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CryptoKeyPair> for Any {
    fn from(s: &CryptoKeyPair) -> Any {
        s.inner.clone()
    }
}

impl CryptoKeyPair {
    pub fn public_key(&self) -> CryptoKey {
        self.inner.get("publicKey").as_::<CryptoKey>()
    }

    pub fn set_public_key(&mut self, value: &CryptoKey) {
        self.inner.set("publicKey", value);
    }
}
impl CryptoKeyPair {
    pub fn private_key(&self) -> CryptoKey {
        self.inner.get("privateKey").as_::<CryptoKey>()
    }

    pub fn set_private_key(&mut self, value: &CryptoKey) {
        self.inner.set("privateKey", value);
    }
}
