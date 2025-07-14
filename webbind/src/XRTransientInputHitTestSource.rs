use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRTransientInputHitTestSource {
    inner: emlite::Val,
}
impl FromVal for XRTransientInputHitTestSource {
    fn from_val(v: &emlite::Val) -> Self {
        XRTransientInputHitTestSource {
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
impl core::ops::Deref for XRTransientInputHitTestSource {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRTransientInputHitTestSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRTransientInputHitTestSource {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRTransientInputHitTestSource {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRTransientInputHitTestSource> for emlite::Val {
    fn from(s: XRTransientInputHitTestSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRTransientInputHitTestSource);

impl XRTransientInputHitTestSource {
    pub fn cancel(&self) -> jsbind::Undefined {
        self.inner.call("cancel", &[]).as_::<jsbind::Undefined>()
    }
}
