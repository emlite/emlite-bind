use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationPreloadState {
    inner: emlite::Val,
}
impl FromVal for NavigationPreloadState {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationPreloadState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationPreloadState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationPreloadState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationPreloadState {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationPreloadState {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationPreloadState> for emlite::Val {
    fn from(s: NavigationPreloadState) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NavigationPreloadState> for emlite::Val {
    fn from(s: &NavigationPreloadState) -> emlite::Val {
        s.inner.clone()
    }
}

impl NavigationPreloadState {
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl NavigationPreloadState {
    pub fn header_value(&self) -> ByteString {
        self.inner.get("headerValue").as_::<ByteString>()
    }

    pub fn set_header_value(&mut self, value: ByteString) {
        self.inner.set("headerValue", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationPreloadManager {
    inner: emlite::Val,
}
impl FromVal for NavigationPreloadManager {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationPreloadManager {
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
impl core::ops::Deref for NavigationPreloadManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationPreloadManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationPreloadManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationPreloadManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationPreloadManager> for emlite::Val {
    fn from(s: NavigationPreloadManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NavigationPreloadManager> for emlite::Val {
    fn from(s: &NavigationPreloadManager) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigationPreloadManager);

impl NavigationPreloadManager {
    pub fn enable(&self) -> Promise {
        self.inner.call("enable", &[]).as_::<Promise>()
    }
}
impl NavigationPreloadManager {
    pub fn disable(&self) -> Promise {
        self.inner.call("disable", &[]).as_::<Promise>()
    }
}
impl NavigationPreloadManager {
    pub fn set_header_value(&self, value: ByteString) -> Promise {
        self.inner
            .call("setHeaderValue", &[value.into()])
            .as_::<Promise>()
    }
}
impl NavigationPreloadManager {
    pub fn get_state(&self) -> Promise {
        self.inner.call("getState", &[]).as_::<Promise>()
    }
}
