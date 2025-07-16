use super::*;

/// The NavigationDestination class.
/// [`NavigationDestination`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationDestination {
    inner: Any,
}
impl FromVal for NavigationDestination {
    fn from_val(v: &Any) -> Self {
        NavigationDestination {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationDestination {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigationDestination {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigationDestination {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigationDestination> for Any {
    fn from(s: NavigationDestination) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigationDestination> for Any {
    fn from(s: &NavigationDestination) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigationDestination);

impl NavigationDestination {
    /// Getter of the `url` attribute.
    /// [`NavigationDestination.url`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/url)
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
impl NavigationDestination {
    /// Getter of the `key` attribute.
    /// [`NavigationDestination.key`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/key)
    pub fn key(&self) -> String {
        self.inner.get("key").as_::<String>()
    }
}
impl NavigationDestination {
    /// Getter of the `id` attribute.
    /// [`NavigationDestination.id`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/id)
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl NavigationDestination {
    /// Getter of the `index` attribute.
    /// [`NavigationDestination.index`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/index)
    pub fn index(&self) -> i64 {
        self.inner.get("index").as_::<i64>()
    }
}
impl NavigationDestination {
    /// Getter of the `sameDocument` attribute.
    /// [`NavigationDestination.sameDocument`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/sameDocument)
    pub fn same_document(&self) -> bool {
        self.inner.get("sameDocument").as_::<bool>()
    }
}
impl NavigationDestination {
    /// The getState method.
    /// [`NavigationDestination.getState`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationDestination/getState)
    pub fn get_state(&self) -> Any {
        self.inner.call("getState", &[]).as_::<Any>()
    }
}
