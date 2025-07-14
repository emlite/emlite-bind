use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<SubtleCrypto> for emlite::Val {
    fn from(s: SubtleCrypto) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SubtleCrypto {
    pub fn encrypt(
        &self,
        algorithm: jsbind::Any,
        key: CryptoKey,
        data: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call("encrypt", &[algorithm.into(), key.into(), data.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn decrypt(
        &self,
        algorithm: jsbind::Any,
        key: CryptoKey,
        data: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call("decrypt", &[algorithm.into(), key.into(), data.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn sign(
        &self,
        algorithm: jsbind::Any,
        key: CryptoKey,
        data: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call("sign", &[algorithm.into(), key.into(), data.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn verify(
        &self,
        algorithm: jsbind::Any,
        key: CryptoKey,
        signature: jsbind::Any,
        data: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "verify",
                &[algorithm.into(), key.into(), signature.into(), data.into()],
            )
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn digest(&self, algorithm: jsbind::Any, data: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("digest", &[algorithm.into(), data.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn generate_key(
        &self,
        algorithm: jsbind::Any,
        extractable: bool,
        key_usages: jsbind::Sequence<KeyUsage>,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "generateKey",
                &[algorithm.into(), extractable.into(), key_usages.into()],
            )
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn derive_key(
        &self,
        algorithm: jsbind::Any,
        base_key: CryptoKey,
        derived_key_type: jsbind::Any,
        extractable: bool,
        key_usages: jsbind::Sequence<KeyUsage>,
    ) -> jsbind::Promise {
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
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn derive_bits0(&self, algorithm: jsbind::Any, base_key: CryptoKey) -> jsbind::Promise {
        self.inner
            .call("deriveBits", &[algorithm.into(), base_key.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn derive_bits1(
        &self,
        algorithm: jsbind::Any,
        base_key: CryptoKey,
        length: u32,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "deriveBits",
                &[algorithm.into(), base_key.into(), length.into()],
            )
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn import_key(
        &self,
        format: KeyFormat,
        key_data: jsbind::Any,
        algorithm: jsbind::Any,
        extractable: bool,
        key_usages: jsbind::Sequence<KeyUsage>,
    ) -> jsbind::Promise {
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
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn export_key(&self, format: KeyFormat, key: CryptoKey) -> jsbind::Promise {
        self.inner
            .call("exportKey", &[format.into(), key.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn wrap_key(
        &self,
        format: KeyFormat,
        key: CryptoKey,
        wrapping_key: CryptoKey,
        wrap_algorithm: jsbind::Any,
    ) -> jsbind::Promise {
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
            .as_::<jsbind::Promise>()
    }
}
impl SubtleCrypto {
    pub fn unwrap_key(
        &self,
        format: KeyFormat,
        wrapped_key: jsbind::Any,
        unwrapping_key: CryptoKey,
        unwrap_algorithm: jsbind::Any,
        unwrapped_key_algorithm: jsbind::Any,
        extractable: bool,
        key_usages: jsbind::Sequence<KeyUsage>,
    ) -> jsbind::Promise {
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
            .as_::<jsbind::Promise>()
    }
}
