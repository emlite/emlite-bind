use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<NavigationHistoryEntry> for emlite::Val {
    fn from(s: NavigationHistoryEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationHistoryEntry {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl NavigationHistoryEntry {
    pub fn key(&self) -> jsbind::DOMString {
        self.inner.get("key").as_::<jsbind::DOMString>()
    }
}
impl NavigationHistoryEntry {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
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
    pub fn get_state(&self) -> jsbind::Any {
        self.inner.call("getState", &[]).as_::<jsbind::Any>()
    }
}
impl NavigationHistoryEntry {
    pub fn ondispose(&self) -> jsbind::Any {
        self.inner.get("ondispose").as_::<jsbind::Any>()
    }

    pub fn set_ondispose(&mut self, value: jsbind::Any) {
        self.inner.set("ondispose", value);
    }
}
