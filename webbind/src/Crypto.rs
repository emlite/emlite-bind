use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for Crypto {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Crypto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Crypto> for emlite::Val {
    fn from(s: Crypto) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
