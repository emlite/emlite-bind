use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FencedFrameConfig {
    inner: emlite::Val,
}
impl FromVal for FencedFrameConfig {
    fn from_val(v: &emlite::Val) -> Self {
        FencedFrameConfig {
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
impl core::ops::Deref for FencedFrameConfig {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FencedFrameConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FencedFrameConfig> for emlite::Val {
    fn from(s: FencedFrameConfig) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FencedFrameConfig {
    pub fn new(url: jsbind::USVString) -> FencedFrameConfig {
        Self {
            inner: emlite::Val::global("FencedFrameConfig")
                .new(&[url.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FencedFrameConfig {
    pub fn set_shared_storage_context(
        &self,
        context_string: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("setSharedStorageContext", &[context_string.into()])
            .as_::<jsbind::Undefined>()
    }
}
