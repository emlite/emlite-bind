use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendGroup {
    inner: emlite::Val,
}
impl FromVal for WebTransportSendGroup {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportSendGroup {
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
impl core::ops::Deref for WebTransportSendGroup {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportSendGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportSendGroup {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportSendGroup {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportSendGroup> for emlite::Val {
    fn from(s: WebTransportSendGroup) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WebTransportSendGroup);

impl WebTransportSendGroup {
    pub fn get_stats(&self) -> Promise {
        self.inner.call("getStats", &[]).as_::<Promise>()
    }
}
