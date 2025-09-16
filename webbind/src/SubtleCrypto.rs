use super::*;

/// The SubtleCrypto class.
/// [`SubtleCrypto`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubtleCrypto {
    inner: Any,
}

impl FromVal for SubtleCrypto {
    fn from_val(v: &Any) -> Self {
        SubtleCrypto {
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

impl core::ops::Deref for SubtleCrypto {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SubtleCrypto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SubtleCrypto {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SubtleCrypto {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SubtleCrypto> for Any {
    fn from(s: SubtleCrypto) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SubtleCrypto> for Any {
    fn from(s: &SubtleCrypto) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SubtleCrypto);

impl SubtleCrypto {
    /// The encrypt method.
    /// [`SubtleCrypto.encrypt`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)
    pub fn encrypt(&self, algorithm: &Any, key: &CryptoKey, data: &Any) -> Promise<ArrayBuffer> {
        self.inner
            .call("encrypt", &[algorithm.into(), key.into(), data.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The decrypt method.
    /// [`SubtleCrypto.decrypt`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)
    pub fn decrypt(&self, algorithm: &Any, key: &CryptoKey, data: &Any) -> Promise<ArrayBuffer> {
        self.inner
            .call("decrypt", &[algorithm.into(), key.into(), data.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The sign method.
    /// [`SubtleCrypto.sign`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)
    pub fn sign(&self, algorithm: &Any, key: &CryptoKey, data: &Any) -> Promise<ArrayBuffer> {
        self.inner
            .call("sign", &[algorithm.into(), key.into(), data.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The verify method.
    /// [`SubtleCrypto.verify`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)
    pub fn verify(
        &self,
        algorithm: &Any,
        key: &CryptoKey,
        signature: &Any,
        data: &Any,
    ) -> Promise<bool> {
        self.inner
            .call(
                "verify",
                &[algorithm.into(), key.into(), signature.into(), data.into()],
            )
            .as_::<Promise<bool>>()
    }
}
impl SubtleCrypto {
    /// The digest method.
    /// [`SubtleCrypto.digest`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)
    pub fn digest(&self, algorithm: &Any, data: &Any) -> Promise<ArrayBuffer> {
        self.inner
            .call("digest", &[algorithm.into(), data.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The generateKey method.
    /// [`SubtleCrypto.generateKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)
    pub fn generate_key(
        &self,
        algorithm: &Any,
        extractable: bool,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<Any> {
        self.inner
            .call(
                "generateKey",
                &[algorithm.into(), extractable.into(), key_usages.into()],
            )
            .as_::<Promise<Any>>()
    }
}
impl SubtleCrypto {
    /// The deriveKey method.
    /// [`SubtleCrypto.deriveKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)
    pub fn derive_key(
        &self,
        algorithm: &Any,
        base_key: &CryptoKey,
        derived_key_type: &Any,
        extractable: bool,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<CryptoKey> {
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
            .as_::<Promise<CryptoKey>>()
    }
}
impl SubtleCrypto {
    /// The deriveBits method.
    /// [`SubtleCrypto.deriveBits`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)
    pub fn derive_bits(&self, algorithm: &Any, base_key: &CryptoKey) -> Promise<ArrayBuffer> {
        self.inner
            .call("deriveBits", &[algorithm.into(), base_key.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The deriveBits method.
    /// [`SubtleCrypto.deriveBits`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)
    pub fn derive_bits_with_length(
        &self,
        algorithm: &Any,
        base_key: &CryptoKey,
        length: u32,
    ) -> Promise<ArrayBuffer> {
        self.inner
            .call(
                "deriveBits",
                &[algorithm.into(), base_key.into(), length.into()],
            )
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The importKey method.
    /// [`SubtleCrypto.importKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)
    pub fn import_key(
        &self,
        format: &KeyFormat,
        key_data: &Any,
        algorithm: &Any,
        extractable: bool,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<CryptoKey> {
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
            .as_::<Promise<CryptoKey>>()
    }
}
impl SubtleCrypto {
    /// The exportKey method.
    /// [`SubtleCrypto.exportKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/exportKey)
    pub fn export_key(&self, format: &KeyFormat, key: &CryptoKey) -> Promise<Any> {
        self.inner
            .call("exportKey", &[format.into(), key.into()])
            .as_::<Promise<Any>>()
    }
}
impl SubtleCrypto {
    /// The wrapKey method.
    /// [`SubtleCrypto.wrapKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)
    pub fn wrap_key(
        &self,
        format: &KeyFormat,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &Any,
    ) -> Promise<ArrayBuffer> {
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
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The unwrapKey method.
    /// [`SubtleCrypto.unwrapKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)
    pub fn unwrap_key(
        &self,
        format: &KeyFormat,
        wrapped_key: &Any,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &Any,
        unwrapped_key_algorithm: &Any,
        extractable: bool,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<CryptoKey> {
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
            .as_::<Promise<CryptoKey>>()
    }
}
impl SubtleCrypto {
    /// The encapsulateKey method.
    /// [`SubtleCrypto.encapsulateKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encapsulateKey)
    pub fn encapsulate_key(
        &self,
        encapsulation_algorithm: &Any,
        encapsulation_key: &CryptoKey,
        shared_key_algorithm: &Any,
        extractable: bool,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<EncapsulatedKey> {
        self.inner
            .call(
                "encapsulateKey",
                &[
                    encapsulation_algorithm.into(),
                    encapsulation_key.into(),
                    shared_key_algorithm.into(),
                    extractable.into(),
                    key_usages.into(),
                ],
            )
            .as_::<Promise<EncapsulatedKey>>()
    }
}
impl SubtleCrypto {
    /// The encapsulateBits method.
    /// [`SubtleCrypto.encapsulateBits`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encapsulateBits)
    pub fn encapsulate_bits(
        &self,
        encapsulation_algorithm: &Any,
        encapsulation_key: &CryptoKey,
    ) -> Promise<EncapsulatedBits> {
        self.inner
            .call(
                "encapsulateBits",
                &[encapsulation_algorithm.into(), encapsulation_key.into()],
            )
            .as_::<Promise<EncapsulatedBits>>()
    }
}
impl SubtleCrypto {
    /// The decapsulateKey method.
    /// [`SubtleCrypto.decapsulateKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decapsulateKey)
    pub fn decapsulate_key(
        &self,
        decapsulation_algorithm: &Any,
        decapsulation_key: &CryptoKey,
        ciphertext: &Any,
        shared_key_algorithm: &Any,
        extractable: bool,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<CryptoKey> {
        self.inner
            .call(
                "decapsulateKey",
                &[
                    decapsulation_algorithm.into(),
                    decapsulation_key.into(),
                    ciphertext.into(),
                    shared_key_algorithm.into(),
                    extractable.into(),
                    key_usages.into(),
                ],
            )
            .as_::<Promise<CryptoKey>>()
    }
}
impl SubtleCrypto {
    /// The decapsulateBits method.
    /// [`SubtleCrypto.decapsulateBits`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decapsulateBits)
    pub fn decapsulate_bits(
        &self,
        decapsulation_algorithm: &Any,
        decapsulation_key: &CryptoKey,
        ciphertext: &Any,
    ) -> Promise<ArrayBuffer> {
        self.inner
            .call(
                "decapsulateBits",
                &[
                    decapsulation_algorithm.into(),
                    decapsulation_key.into(),
                    ciphertext.into(),
                ],
            )
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl SubtleCrypto {
    /// The getPublicKey method.
    /// [`SubtleCrypto.getPublicKey`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/getPublicKey)
    pub fn get_public_key(
        &self,
        key: &CryptoKey,
        key_usages: &TypedArray<KeyUsage>,
    ) -> Promise<CryptoKey> {
        self.inner
            .call("getPublicKey", &[key.into(), key_usages.into()])
            .as_::<Promise<CryptoKey>>()
    }
}
impl SubtleCrypto {
    /// The supports method.
    /// [`SubtleCrypto.supports`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/supports)
    pub fn supports(operation: &JsString, algorithm: &Any) -> bool {
        Any::global("SubtleCrypto")
            .call("supports", &[operation.into(), algorithm.into()])
            .as_::<bool>()
    }
}
impl SubtleCrypto {
    /// The supports method.
    /// [`SubtleCrypto.supports`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/supports)
    pub fn supports_with_length(operation: &JsString, algorithm: &Any, length: u32) -> bool {
        Any::global("SubtleCrypto")
            .call(
                "supports",
                &[operation.into(), algorithm.into(), length.into()],
            )
            .as_::<bool>()
    }
}
impl SubtleCrypto {
    /// The supports method.
    /// [`SubtleCrypto.supports`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/supports)
    pub fn supports_with_operation_and_algorithm_and_additional_algorithm(
        operation: &JsString,
        algorithm: &Any,
        additional_algorithm: &Any,
    ) -> bool {
        Any::global("SubtleCrypto")
            .call(
                "supports",
                &[
                    operation.into(),
                    algorithm.into(),
                    additional_algorithm.into(),
                ],
            )
            .as_::<bool>()
    }
}
