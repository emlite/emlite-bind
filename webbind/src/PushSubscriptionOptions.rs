use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushSubscriptionOptions {
    inner: emlite::Val,
}
impl FromVal for PushSubscriptionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PushSubscriptionOptions {
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
impl core::ops::Deref for PushSubscriptionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushSubscriptionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushSubscriptionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushSubscriptionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PushSubscriptionOptions> for emlite::Val {
    fn from(s: PushSubscriptionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PushSubscriptionOptions> for emlite::Val {
    fn from(s: &PushSubscriptionOptions) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PushSubscriptionOptions);

impl PushSubscriptionOptions {
    pub fn user_visible_only(&self) -> bool {
        self.inner.get("userVisibleOnly").as_::<bool>()
    }
}
impl PushSubscriptionOptions {
    pub fn application_server_key(&self) -> ArrayBuffer {
        self.inner.get("applicationServerKey").as_::<ArrayBuffer>()
    }
}
