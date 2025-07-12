use super::*;

#[derive(Clone, Debug)]
pub struct WGSLLanguageFeatures {
    inner: emlite::Val,
}
impl FromVal for WGSLLanguageFeatures {
    fn from_val(v: &emlite::Val) -> Self {
        WGSLLanguageFeatures {
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
impl std::ops::Deref for WGSLLanguageFeatures {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WGSLLanguageFeatures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WGSLLanguageFeatures> for emlite::Val {
    fn from(s: WGSLLanguageFeatures) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
