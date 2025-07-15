use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CryptoKey {
    inner: emlite::Val,
}
impl FromVal for CryptoKey {
    fn from_val(v: &emlite::Val) -> Self {
        CryptoKey {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CryptoKey {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CryptoKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CryptoKey {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CryptoKey {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CryptoKey> for emlite::Val {
    fn from(s: CryptoKey) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CryptoKey> for emlite::Val {
    fn from(s: &CryptoKey) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CryptoKey);

impl CryptoKey {
    pub fn type_(&self) -> KeyType {
        self.inner.get("type").as_::<KeyType>()
    }
}
impl CryptoKey {
    pub fn extractable(&self) -> bool {
        self.inner.get("extractable").as_::<bool>()
    }
}
impl CryptoKey {
    pub fn algorithm(&self) -> Object {
        self.inner.get("algorithm").as_::<Object>()
    }
}
impl CryptoKey {
    pub fn usages(&self) -> Object {
        self.inner.get("usages").as_::<Object>()
    }
}
