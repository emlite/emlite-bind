use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaHashedKeyGenParams {
    inner: Any,
}
impl FromVal for RsaHashedKeyGenParams {
    fn from_val(v: &Any) -> Self {
        RsaHashedKeyGenParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RsaHashedKeyGenParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RsaHashedKeyGenParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RsaHashedKeyGenParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RsaHashedKeyGenParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RsaHashedKeyGenParams> for Any {
    fn from(s: RsaHashedKeyGenParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RsaHashedKeyGenParams> for Any {
    fn from(s: &RsaHashedKeyGenParams) -> Any {
        s.inner.clone()
    }
}

impl RsaHashedKeyGenParams {
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
