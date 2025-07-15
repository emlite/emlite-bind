use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowClient {
    inner: Client,
}
impl FromVal for WindowClient {
    fn from_val(v: &emlite::Val) -> Self {
        WindowClient {
            inner: Client::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WindowClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WindowClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WindowClient {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WindowClient {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WindowClient> for emlite::Val {
    fn from(s: WindowClient) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WindowClient> for emlite::Val {
    fn from(s: &WindowClient) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WindowClient);

impl WindowClient {
    pub fn visibility_state(&self) -> DocumentVisibilityState {
        self.inner
            .get("visibilityState")
            .as_::<DocumentVisibilityState>()
    }
}
impl WindowClient {
    pub fn focused(&self) -> bool {
        self.inner.get("focused").as_::<bool>()
    }
}
impl WindowClient {
    pub fn ancestor_origins(&self) -> FrozenArray<String> {
        self.inner
            .get("ancestorOrigins")
            .as_::<FrozenArray<String>>()
    }
}
impl WindowClient {
    pub fn focus(&self) -> Promise {
        self.inner.call("focus", &[]).as_::<Promise>()
    }
}
impl WindowClient {
    pub fn navigate(&self, url: &str) -> Promise {
        self.inner.call("navigate", &[url.into()]).as_::<Promise>()
    }
}
