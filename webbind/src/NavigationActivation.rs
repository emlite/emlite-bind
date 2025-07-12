use super::*;

#[derive(Clone, Debug)]
pub struct NavigationActivation {
    inner: emlite::Val,
}
impl FromVal for NavigationActivation {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationActivation {
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
impl std::ops::Deref for NavigationActivation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigationActivation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigationActivation> for emlite::Val {
    fn from(s: NavigationActivation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationActivation {
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }
}
impl NavigationActivation {
    pub fn entry(&self) -> NavigationHistoryEntry {
        self.inner.get("entry").as_::<NavigationHistoryEntry>()
    }
}
impl NavigationActivation {
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
