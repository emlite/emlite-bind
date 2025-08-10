use super::*;

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
    pub fn kty(&self) -> JsString {
        self.inner.get("kty").as_::<JsString>()
    }

    pub fn set_kty(&mut self, value: &JsString) {
        self.inner.set("kty", value);
    }
}
impl JsonWebKey {
    pub fn use_(&self) -> JsString {
        self.inner.get("use").as_::<JsString>()
    }

    pub fn set_use_(&mut self, value: &JsString) {
        self.inner.set("use", value);
    }
}
impl JsonWebKey {
    pub fn key_ops(&self) -> TypedArray<JsString> {
        self.inner.get("key_ops").as_::<TypedArray<JsString>>()
    }

    pub fn set_key_ops(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("key_ops", value);
    }
}
impl JsonWebKey {
    pub fn alg(&self) -> JsString {
        self.inner.get("alg").as_::<JsString>()
    }

    pub fn set_alg(&mut self, value: &JsString) {
        self.inner.set("alg", value);
    }
}
impl JsonWebKey {
    pub fn ext(&self) -> bool {
        self.inner.get("ext").as_::<bool>()
    }

    pub fn set_ext(&mut self, value: bool) {
        self.inner.set("ext", value);
    }
}
impl JsonWebKey {
    pub fn crv(&self) -> JsString {
        self.inner.get("crv").as_::<JsString>()
    }

    pub fn set_crv(&mut self, value: &JsString) {
        self.inner.set("crv", value);
    }
}
impl JsonWebKey {
    pub fn x(&self) -> JsString {
        self.inner.get("x").as_::<JsString>()
    }

    pub fn set_x(&mut self, value: &JsString) {
        self.inner.set("x", value);
    }
}
impl JsonWebKey {
    pub fn y(&self) -> JsString {
        self.inner.get("y").as_::<JsString>()
    }

    pub fn set_y(&mut self, value: &JsString) {
        self.inner.set("y", value);
    }
}
impl JsonWebKey {
    pub fn d(&self) -> JsString {
        self.inner.get("d").as_::<JsString>()
    }

    pub fn set_d(&mut self, value: &JsString) {
        self.inner.set("d", value);
    }
}
impl JsonWebKey {
    pub fn n(&self) -> JsString {
        self.inner.get("n").as_::<JsString>()
    }

    pub fn set_n(&mut self, value: &JsString) {
        self.inner.set("n", value);
    }
}
impl JsonWebKey {
    pub fn e(&self) -> JsString {
        self.inner.get("e").as_::<JsString>()
    }

    pub fn set_e(&mut self, value: &JsString) {
        self.inner.set("e", value);
    }
}
impl JsonWebKey {
    pub fn p(&self) -> JsString {
        self.inner.get("p").as_::<JsString>()
    }

    pub fn set_p(&mut self, value: &JsString) {
        self.inner.set("p", value);
    }
}
impl JsonWebKey {
    pub fn q(&self) -> JsString {
        self.inner.get("q").as_::<JsString>()
    }

    pub fn set_q(&mut self, value: &JsString) {
        self.inner.set("q", value);
    }
}
impl JsonWebKey {
    pub fn dp(&self) -> JsString {
        self.inner.get("dp").as_::<JsString>()
    }

    pub fn set_dp(&mut self, value: &JsString) {
        self.inner.set("dp", value);
    }
}
impl JsonWebKey {
    pub fn dq(&self) -> JsString {
        self.inner.get("dq").as_::<JsString>()
    }

    pub fn set_dq(&mut self, value: &JsString) {
        self.inner.set("dq", value);
    }
}
impl JsonWebKey {
    pub fn qi(&self) -> JsString {
        self.inner.get("qi").as_::<JsString>()
    }

    pub fn set_qi(&mut self, value: &JsString) {
        self.inner.set("qi", value);
    }
}
impl JsonWebKey {
    pub fn oth(&self) -> TypedArray<RsaOtherPrimesInfo> {
        self.inner
            .get("oth")
            .as_::<TypedArray<RsaOtherPrimesInfo>>()
    }

    pub fn set_oth(&mut self, value: &TypedArray<RsaOtherPrimesInfo>) {
        self.inner.set("oth", value);
    }
}
impl JsonWebKey {
    pub fn k(&self) -> JsString {
        self.inner.get("k").as_::<JsString>()
    }

    pub fn set_k(&mut self, value: &JsString) {
        self.inner.set("k", value);
    }
}
