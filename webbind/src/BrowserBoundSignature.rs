use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BrowserBoundSignature {
    inner: Any,
}
impl FromVal for BrowserBoundSignature {
    fn from_val(v: &Any) -> Self {
        BrowserBoundSignature { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BrowserBoundSignature {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BrowserBoundSignature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BrowserBoundSignature {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BrowserBoundSignature {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BrowserBoundSignature> for Any {
    fn from(s: BrowserBoundSignature) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BrowserBoundSignature> for Any {
    fn from(s: &BrowserBoundSignature) -> Any {
        s.inner.clone()
    }
}

impl BrowserBoundSignature {
    pub fn signature(&self) -> ArrayBuffer {
        self.inner.get("signature").as_::<ArrayBuffer>()
    }

    pub fn set_signature(&mut self, value: &ArrayBuffer) {
        self.inner.set("signature", value);
    }
}
