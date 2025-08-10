use super::*;

/// The RsaOtherPrimesInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaOtherPrimesInfo {
    inner: Any,
}

impl FromVal for RsaOtherPrimesInfo {
    fn from_val(v: &Any) -> Self {
        RsaOtherPrimesInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RsaOtherPrimesInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RsaOtherPrimesInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RsaOtherPrimesInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RsaOtherPrimesInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RsaOtherPrimesInfo> for Any {
    fn from(s: RsaOtherPrimesInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RsaOtherPrimesInfo> for Any {
    fn from(s: &RsaOtherPrimesInfo) -> Any {
        s.inner.clone()
    }
}

impl RsaOtherPrimesInfo {
    /// Getter of the `r` attribute.
    pub fn r(&self) -> JsString {
        self.inner.get("r").as_::<JsString>()
    }

    /// Setter of the `r` attribute.
    pub fn set_r(&mut self, value: &JsString) {
        self.inner.set("r", value);
    }
}
impl RsaOtherPrimesInfo {
    /// Getter of the `d` attribute.
    pub fn d(&self) -> JsString {
        self.inner.get("d").as_::<JsString>()
    }

    /// Setter of the `d` attribute.
    pub fn set_d(&mut self, value: &JsString) {
        self.inner.set("d", value);
    }
}
impl RsaOtherPrimesInfo {
    /// Getter of the `t` attribute.
    pub fn t(&self) -> JsString {
        self.inner.get("t").as_::<JsString>()
    }

    /// Setter of the `t` attribute.
    pub fn set_t(&mut self, value: &JsString) {
        self.inner.set("t", value);
    }
}
