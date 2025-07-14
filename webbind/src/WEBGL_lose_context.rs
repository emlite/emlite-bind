use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_lose_context {
    inner: emlite::Val,
}
impl FromVal for WEBGL_lose_context {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_lose_context {
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
impl core::ops::Deref for WEBGL_lose_context {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_lose_context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_lose_context {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_lose_context {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_lose_context> for emlite::Val {
    fn from(s: WEBGL_lose_context) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_lose_context);

impl WEBGL_lose_context {
    pub fn lose_context(&self) -> jsbind::Undefined {
        self.inner
            .call("loseContext", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl WEBGL_lose_context {
    pub fn restore_context(&self) -> jsbind::Undefined {
        self.inner
            .call("restoreContext", &[])
            .as_::<jsbind::Undefined>()
    }
}
