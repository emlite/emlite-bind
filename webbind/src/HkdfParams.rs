use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HkdfParams {
    inner: Any,
}
impl FromVal for HkdfParams {
    fn from_val(v: &Any) -> Self {
        HkdfParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HkdfParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HkdfParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HkdfParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HkdfParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HkdfParams> for Any {
    fn from(s: HkdfParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HkdfParams> for Any {
    fn from(s: &HkdfParams) -> Any {
        s.inner.clone()
    }
}

impl HkdfParams {
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
impl HkdfParams {
    pub fn salt(&self) -> Any {
        self.inner.get("salt").as_::<Any>()
    }

    pub fn set_salt(&mut self, value: &Any) {
        self.inner.set("salt", value);
    }
}
impl HkdfParams {
    pub fn info(&self) -> Any {
        self.inner.get("info").as_::<Any>()
    }

    pub fn set_info(&mut self, value: &Any) {
        self.inner.set("info", value);
    }
}
