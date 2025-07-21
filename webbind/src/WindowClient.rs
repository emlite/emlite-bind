use super::*;

/// The WindowClient class.
/// [`WindowClient`](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowClient {
    inner: Client,
}
impl FromVal for WindowClient {
    fn from_val(v: &Any) -> Self {
        WindowClient {
            inner: Client::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for WindowClient {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WindowClient {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WindowClient> for Any {
    fn from(s: WindowClient) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WindowClient> for Any {
    fn from(s: &WindowClient) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WindowClient);

impl WindowClient {
    /// Getter of the `visibilityState` attribute.
    /// [`WindowClient.visibilityState`](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/visibilityState)
    pub fn visibility_state(&self) -> DocumentVisibilityState {
        self.inner
            .get("visibilityState")
            .as_::<DocumentVisibilityState>()
    }
}
impl WindowClient {
    /// Getter of the `focused` attribute.
    /// [`WindowClient.focused`](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focused)
    pub fn focused(&self) -> bool {
        self.inner.get("focused").as_::<bool>()
    }
}
impl WindowClient {
    /// Getter of the `ancestorOrigins` attribute.
    /// [`WindowClient.ancestorOrigins`](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/ancestorOrigins)
    pub fn ancestor_origins(&self) -> FrozenArray<String> {
        self.inner
            .get("ancestorOrigins")
            .as_::<FrozenArray<String>>()
    }
}
impl WindowClient {
    /// The focus method.
    /// [`WindowClient.focus`](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focus)
    pub fn focus(&self) -> Promise<WindowClient> {
        self.inner.call("focus", &[]).as_::<Promise<WindowClient>>()
    }
}
impl WindowClient {
    /// The navigate method.
    /// [`WindowClient.navigate`](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/navigate)
    pub fn navigate(&self, url: &str) -> Promise<WindowClient> {
        self.inner
            .call("navigate", &[url.into()])
            .as_::<Promise<WindowClient>>()
    }
}
