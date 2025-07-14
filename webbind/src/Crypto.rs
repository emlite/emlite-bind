use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Crypto {
    inner: emlite::Val,
}
impl FromVal for Crypto {
    fn from_val(v: &emlite::Val) -> Self {
        Crypto {
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
impl core::ops::Deref for Crypto {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Crypto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Crypto {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Crypto {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Crypto> for emlite::Val {
    fn from(s: Crypto) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Crypto);

impl Crypto {
    pub fn subtle(&self) -> SubtleCrypto {
        self.inner.get("subtle").as_::<SubtleCrypto>()
    }
}
impl Crypto {
    pub fn get_random_values(&self, array: jsbind::Any) -> jsbind::Any {
        self.inner
            .call("getRandomValues", &[array.into()])
            .as_::<jsbind::Any>()
    }
}
impl Crypto {
    pub fn random_uuid(&self) -> jsbind::DOMString {
        self.inner
            .call("randomUUID", &[])
            .as_::<jsbind::DOMString>()
    }
}
