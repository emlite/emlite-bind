use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for WindowClient {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WindowClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WindowClient> for emlite::Val {
    fn from(s: WindowClient) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn ancestor_origins(&self) -> jsbind::FrozenArray<jsbind::USVString> {
        self.inner
            .get("ancestorOrigins")
            .as_::<jsbind::FrozenArray<jsbind::USVString>>()
    }
}
impl WindowClient {
    pub fn focus(&self) -> jsbind::Promise {
        self.inner.call("focus", &[]).as_::<jsbind::Promise>()
    }
}
impl WindowClient {
    pub fn navigate(&self, url: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("navigate", &[url.into()])
            .as_::<jsbind::Promise>()
    }
}
