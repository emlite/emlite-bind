use super::*;

/// The NavigationHistoryEntry class.
/// [`NavigationHistoryEntry`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationHistoryEntry {
    inner: EventTarget,
}

impl FromVal for NavigationHistoryEntry {
    fn from_val(v: &Any) -> Self {
        NavigationHistoryEntry {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationHistoryEntry {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationHistoryEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationHistoryEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationHistoryEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationHistoryEntry> for Any {
    fn from(s: NavigationHistoryEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationHistoryEntry> for Any {
    fn from(s: &NavigationHistoryEntry) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NavigationHistoryEntry);

impl NavigationHistoryEntry {
    /// Getter of the `url` attribute.
    /// [`NavigationHistoryEntry.url`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl NavigationHistoryEntry {
    /// Getter of the `key` attribute.
    /// [`NavigationHistoryEntry.key`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/key)
    pub fn key(&self) -> JsString {
        self.inner.get("key").as_::<JsString>()
    }
}
impl NavigationHistoryEntry {
    /// Getter of the `id` attribute.
    /// [`NavigationHistoryEntry.id`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl NavigationHistoryEntry {
    /// Getter of the `index` attribute.
    /// [`NavigationHistoryEntry.index`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/index)
    pub fn index(&self) -> i64 {
        self.inner.get("index").as_::<i64>()
    }
}
impl NavigationHistoryEntry {
    /// Getter of the `sameDocument` attribute.
    /// [`NavigationHistoryEntry.sameDocument`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/sameDocument)
    pub fn same_document(&self) -> bool {
        self.inner.get("sameDocument").as_::<bool>()
    }
}
impl NavigationHistoryEntry {
    /// The getState method.
    /// [`NavigationHistoryEntry.getState`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/getState)
    pub fn get_state(&self) -> Any {
        self.inner.call("getState", &[]).as_::<Any>()
    }
}
impl NavigationHistoryEntry {
    /// Getter of the `ondispose` attribute.
    /// [`NavigationHistoryEntry.ondispose`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/ondispose)
    pub fn ondispose(&self) -> Any {
        self.inner.get("ondispose").as_::<Any>()
    }

    /// Setter of the `ondispose` attribute.
    /// [`NavigationHistoryEntry.ondispose`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationHistoryEntry/ondispose)
    pub fn set_ondispose(&mut self, value: &Any) {
        self.inner.set("ondispose", value);
    }
}
