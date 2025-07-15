use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubtleCrypto {
    inner: emlite::Val,
}
impl FromVal for SubtleCrypto {
    fn from_val(v: &emlite::Val) -> Self {
        SubtleCrypto {
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
impl core::ops::Deref for SubtleCrypto {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SubtleCrypto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SubtleCrypto {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SubtleCrypto {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SubtleCrypto> for emlite::Val {
    fn from(s: SubtleCrypto) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SubtleCrypto);

impl SubtleCrypto {
    pub fn encrypt(&self, algorithm: Any, key: CryptoKey, data: Any) -> Promise {
        self.inner
            .call("encrypt", &[algorithm.into(), key.into(), data.into()])
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn decrypt(&self, algorithm: Any, key: CryptoKey, data: Any) -> Promise {
        self.inner
            .call("decrypt", &[algorithm.into(), key.into(), data.into()])
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn sign(&self, algorithm: Any, key: CryptoKey, data: Any) -> Promise {
        self.inner
            .call("sign", &[algorithm.into(), key.into(), data.into()])
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn verify(&self, algorithm: Any, key: CryptoKey, signature: Any, data: Any) -> Promise {
        self.inner
            .call(
                "verify",
                &[algorithm.into(), key.into(), signature.into(), data.into()],
            )
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn digest(&self, algorithm: Any, data: Any) -> Promise {
        self.inner
            .call("digest", &[algorithm.into(), data.into()])
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn generate_key(
        &self,
        algorithm: Any,
        extractable: bool,
        key_usages: Sequence<KeyUsage>,
    ) -> Promise {
        self.inner
            .call(
                "generateKey",
                &[algorithm.into(), extractable.into(), key_usages.into()],
            )
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn derive_key(
        &self,
        algorithm: Any,
        base_key: CryptoKey,
        derived_key_type: Any,
        extractable: bool,
        key_usages: Sequence<KeyUsage>,
    ) -> Promise {
        self.inner
            .call(
                "deriveKey",
                &[
                    algorithm.into(),
                    base_key.into(),
                    derived_key_type.into(),
                    extractable.into(),
                    key_usages.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn derive_bits0(&self, algorithm: Any, base_key: CryptoKey) -> Promise {
        self.inner
            .call("deriveBits", &[algorithm.into(), base_key.into()])
            .as_::<Promise>()
    }

    pub fn derive_bits1(&self, algorithm: Any, base_key: CryptoKey, length: u32) -> Promise {
        self.inner
            .call(
                "deriveBits",
                &[algorithm.into(), base_key.into(), length.into()],
            )
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn import_key(
        &self,
        format: KeyFormat,
        key_data: Any,
        algorithm: Any,
        extractable: bool,
        key_usages: Sequence<KeyUsage>,
    ) -> Promise {
        self.inner
            .call(
                "importKey",
                &[
                    format.into(),
                    key_data.into(),
                    algorithm.into(),
                    extractable.into(),
                    key_usages.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn export_key(&self, format: KeyFormat, key: CryptoKey) -> Promise {
        self.inner
            .call("exportKey", &[format.into(), key.into()])
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn wrap_key(
        &self,
        format: KeyFormat,
        key: CryptoKey,
        wrapping_key: CryptoKey,
        wrap_algorithm: Any,
    ) -> Promise {
        self.inner
            .call(
                "wrapKey",
                &[
                    format.into(),
                    key.into(),
                    wrapping_key.into(),
                    wrap_algorithm.into(),
                ],
            )
            .as_::<Promise>()
    }
}
impl SubtleCrypto {
    pub fn unwrap_key(
        &self,
        format: KeyFormat,
        wrapped_key: Any,
        unwrapping_key: CryptoKey,
        unwrap_algorithm: Any,
        unwrapped_key_algorithm: Any,
        extractable: bool,
        key_usages: Sequence<KeyUsage>,
    ) -> Promise {
        self.inner
            .call(
                "unwrapKey",
                &[
                    format.into(),
                    wrapped_key.into(),
                    unwrapping_key.into(),
                    unwrap_algorithm.into(),
                    unwrapped_key_algorithm.into(),
                    extractable.into(),
                    key_usages.into(),
                ],
            )
            .as_::<Promise>()
    }
}
