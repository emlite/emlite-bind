use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaPssParams {
    inner: Any,
}
impl FromVal for RsaPssParams {
    fn from_val(v: &Any) -> Self {
        RsaPssParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RsaPssParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RsaPssParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RsaPssParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RsaPssParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RsaPssParams> for Any {
    fn from(s: RsaPssParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RsaPssParams> for Any {
    fn from(s: &RsaPssParams) -> Any {
        s.inner.clone()
    }
}

impl RsaPssParams {
    pub fn salt_length(&self) -> u32 {
        self.inner.get("saltLength").as_::<u32>()
    }

    pub fn set_salt_length(&mut self, value: u32) {
        self.inner.set("saltLength", value);
    }
}
