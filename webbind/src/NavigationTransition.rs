use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationTransition {
    inner: emlite::Val,
}
impl FromVal for NavigationTransition {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationTransition {
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
impl core::ops::Deref for NavigationTransition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationTransition {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationTransition {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationTransition> for emlite::Val {
    fn from(s: NavigationTransition) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NavigationTransition);

impl NavigationTransition {
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
impl NavigationTransition {
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }
}
impl NavigationTransition {
    pub fn finished(&self) -> jsbind::Promise {
        self.inner.get("finished").as_::<jsbind::Promise>()
    }
}
