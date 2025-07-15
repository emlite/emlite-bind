use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationHistoryEntry {
    inner: EventTarget,
}
impl FromVal for NavigationHistoryEntry {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationHistoryEntry {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for NavigationHistoryEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationHistoryEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationHistoryEntry> for emlite::Val {
    fn from(s: NavigationHistoryEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NavigationHistoryEntry);

impl NavigationHistoryEntry {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }
}
impl NavigationHistoryEntry {
    pub fn key(&self) -> DOMString {
        self.inner.get("key").as_::<DOMString>()
    }
}
impl NavigationHistoryEntry {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
impl NavigationHistoryEntry {
    pub fn index(&self) -> i64 {
        self.inner.get("index").as_::<i64>()
    }
}
impl NavigationHistoryEntry {
    pub fn same_document(&self) -> bool {
        self.inner.get("sameDocument").as_::<bool>()
    }
}
impl NavigationHistoryEntry {
    pub fn get_state(&self) -> Any {
        self.inner.call("getState", &[]).as_::<Any>()
    }
}
impl NavigationHistoryEntry {
    pub fn ondispose(&self) -> Any {
        self.inner.get("ondispose").as_::<Any>()
    }

    pub fn set_ondispose(&mut self, value: Any) {
        self.inner.set("ondispose", value);
    }
}
