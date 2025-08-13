use super::*;




/// The CryptoKey class.
/// [`CryptoKey`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CryptoKey {
    inner: Any,
}

impl FromVal for CryptoKey {
    fn from_val(v: &Any) -> Self {
        CryptoKey { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CryptoKey {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CryptoKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CryptoKey {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CryptoKey {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CryptoKey> for Any {
    fn from(s: CryptoKey) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CryptoKey> for Any {
    fn from(s: &CryptoKey) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CryptoKey);


impl CryptoKey {
    /// Getter of the `type` attribute.
    /// [`CryptoKey.type`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/type)
    pub fn type_(&self) -> KeyType {
        self.inner.get("type").as_::<KeyType>()
    }

}
impl CryptoKey {
    /// Getter of the `extractable` attribute.
    /// [`CryptoKey.extractable`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/extractable)
    pub fn extractable(&self) -> bool {
        self.inner.get("extractable").as_::<bool>()
    }

}
impl CryptoKey {
    /// Getter of the `algorithm` attribute.
    /// [`CryptoKey.algorithm`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/algorithm)
    pub fn algorithm(&self) -> Object {
        self.inner.get("algorithm").as_::<Object>()
    }

}
impl CryptoKey {
    /// Getter of the `usages` attribute.
    /// [`CryptoKey.usages`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/usages)
    pub fn usages(&self) -> Object {
        self.inner.get("usages").as_::<Object>()
    }

}
