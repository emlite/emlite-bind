use super::*;




/// The JsonWebKey dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct JsonWebKey {
    inner: Any,
}

impl FromVal for JsonWebKey {
    fn from_val(v: &Any) -> Self {
        JsonWebKey { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for JsonWebKey {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for JsonWebKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for JsonWebKey {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for JsonWebKey {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<JsonWebKey> for Any {
    fn from(s: JsonWebKey) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&JsonWebKey> for Any {
    fn from(s: &JsonWebKey) -> Any {
        s.inner.clone()
    }
}

impl JsonWebKey {
    /// Getter of the `kty` attribute.
    pub fn kty(&self) -> JsString {
        self.inner.get("kty").as_::<JsString>()
    }

    /// Setter of the `kty` attribute.
    pub fn set_kty(&mut self, value: &JsString) {
        self.inner.set("kty", value);
    }
}
impl JsonWebKey {
    /// Getter of the `use` attribute.
    pub fn use_(&self) -> JsString {
        self.inner.get("use").as_::<JsString>()
    }

    /// Setter of the `use` attribute.
    pub fn set_use_(&mut self, value: &JsString) {
        self.inner.set("use", value);
    }
}
impl JsonWebKey {
    /// Getter of the `key_ops` attribute.
    pub fn key_ops(&self) -> TypedArray<JsString> {
        self.inner.get("key_ops").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `key_ops` attribute.
    pub fn set_key_ops(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("key_ops", value);
    }
}
impl JsonWebKey {
    /// Getter of the `alg` attribute.
    pub fn alg(&self) -> JsString {
        self.inner.get("alg").as_::<JsString>()
    }

    /// Setter of the `alg` attribute.
    pub fn set_alg(&mut self, value: &JsString) {
        self.inner.set("alg", value);
    }
}
impl JsonWebKey {
    /// Getter of the `ext` attribute.
    pub fn ext(&self) -> bool {
        self.inner.get("ext").as_::<bool>()
    }

    /// Setter of the `ext` attribute.
    pub fn set_ext(&mut self, value: bool) {
        self.inner.set("ext", value);
    }
}
impl JsonWebKey {
    /// Getter of the `crv` attribute.
    pub fn crv(&self) -> JsString {
        self.inner.get("crv").as_::<JsString>()
    }

    /// Setter of the `crv` attribute.
    pub fn set_crv(&mut self, value: &JsString) {
        self.inner.set("crv", value);
    }
}
impl JsonWebKey {
    /// Getter of the `x` attribute.
    pub fn x(&self) -> JsString {
        self.inner.get("x").as_::<JsString>()
    }

    /// Setter of the `x` attribute.
    pub fn set_x(&mut self, value: &JsString) {
        self.inner.set("x", value);
    }
}
impl JsonWebKey {
    /// Getter of the `y` attribute.
    pub fn y(&self) -> JsString {
        self.inner.get("y").as_::<JsString>()
    }

    /// Setter of the `y` attribute.
    pub fn set_y(&mut self, value: &JsString) {
        self.inner.set("y", value);
    }
}
impl JsonWebKey {
    /// Getter of the `d` attribute.
    pub fn d(&self) -> JsString {
        self.inner.get("d").as_::<JsString>()
    }

    /// Setter of the `d` attribute.
    pub fn set_d(&mut self, value: &JsString) {
        self.inner.set("d", value);
    }
}
impl JsonWebKey {
    /// Getter of the `n` attribute.
    pub fn n(&self) -> JsString {
        self.inner.get("n").as_::<JsString>()
    }

    /// Setter of the `n` attribute.
    pub fn set_n(&mut self, value: &JsString) {
        self.inner.set("n", value);
    }
}
impl JsonWebKey {
    /// Getter of the `e` attribute.
    pub fn e(&self) -> JsString {
        self.inner.get("e").as_::<JsString>()
    }

    /// Setter of the `e` attribute.
    pub fn set_e(&mut self, value: &JsString) {
        self.inner.set("e", value);
    }
}
impl JsonWebKey {
    /// Getter of the `p` attribute.
    pub fn p(&self) -> JsString {
        self.inner.get("p").as_::<JsString>()
    }

    /// Setter of the `p` attribute.
    pub fn set_p(&mut self, value: &JsString) {
        self.inner.set("p", value);
    }
}
impl JsonWebKey {
    /// Getter of the `q` attribute.
    pub fn q(&self) -> JsString {
        self.inner.get("q").as_::<JsString>()
    }

    /// Setter of the `q` attribute.
    pub fn set_q(&mut self, value: &JsString) {
        self.inner.set("q", value);
    }
}
impl JsonWebKey {
    /// Getter of the `dp` attribute.
    pub fn dp(&self) -> JsString {
        self.inner.get("dp").as_::<JsString>()
    }

    /// Setter of the `dp` attribute.
    pub fn set_dp(&mut self, value: &JsString) {
        self.inner.set("dp", value);
    }
}
impl JsonWebKey {
    /// Getter of the `dq` attribute.
    pub fn dq(&self) -> JsString {
        self.inner.get("dq").as_::<JsString>()
    }

    /// Setter of the `dq` attribute.
    pub fn set_dq(&mut self, value: &JsString) {
        self.inner.set("dq", value);
    }
}
impl JsonWebKey {
    /// Getter of the `qi` attribute.
    pub fn qi(&self) -> JsString {
        self.inner.get("qi").as_::<JsString>()
    }

    /// Setter of the `qi` attribute.
    pub fn set_qi(&mut self, value: &JsString) {
        self.inner.set("qi", value);
    }
}
impl JsonWebKey {
    /// Getter of the `oth` attribute.
    pub fn oth(&self) -> TypedArray<RsaOtherPrimesInfo> {
        self.inner.get("oth").as_::<TypedArray<RsaOtherPrimesInfo>>()
    }

    /// Setter of the `oth` attribute.
    pub fn set_oth(&mut self, value: &TypedArray<RsaOtherPrimesInfo>) {
        self.inner.set("oth", value);
    }
}
impl JsonWebKey {
    /// Getter of the `k` attribute.
    pub fn k(&self) -> JsString {
        self.inner.get("k").as_::<JsString>()
    }

    /// Setter of the `k` attribute.
    pub fn set_k(&mut self, value: &JsString) {
        self.inner.set("k", value);
    }
}
