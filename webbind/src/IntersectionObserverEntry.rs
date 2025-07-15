use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntersectionObserverEntry {
    inner: emlite::Val,
}
impl FromVal for IntersectionObserverEntry {
    fn from_val(v: &emlite::Val) -> Self {
        IntersectionObserverEntry {
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
impl core::ops::Deref for IntersectionObserverEntry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IntersectionObserverEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IntersectionObserverEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IntersectionObserverEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IntersectionObserverEntry> for emlite::Val {
    fn from(s: IntersectionObserverEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IntersectionObserverEntry);

impl IntersectionObserverEntry {
    pub fn new(intersection_observer_entry_init: Any) -> IntersectionObserverEntry {
        Self {
            inner: emlite::Val::global("IntersectionObserverEntry")
                .new(&[intersection_observer_entry_init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl IntersectionObserverEntry {
    pub fn time(&self) -> Any {
        self.inner.get("time").as_::<Any>()
    }
}
impl IntersectionObserverEntry {
    pub fn root_bounds(&self) -> DOMRectReadOnly {
        self.inner.get("rootBounds").as_::<DOMRectReadOnly>()
    }
}
impl IntersectionObserverEntry {
    pub fn bounding_client_rect(&self) -> DOMRectReadOnly {
        self.inner
            .get("boundingClientRect")
            .as_::<DOMRectReadOnly>()
    }
}
impl IntersectionObserverEntry {
    pub fn intersection_rect(&self) -> DOMRectReadOnly {
        self.inner.get("intersectionRect").as_::<DOMRectReadOnly>()
    }
}
impl IntersectionObserverEntry {
    pub fn is_intersecting(&self) -> bool {
        self.inner.get("isIntersecting").as_::<bool>()
    }
}
impl IntersectionObserverEntry {
    pub fn is_visible(&self) -> bool {
        self.inner.get("isVisible").as_::<bool>()
    }
}
impl IntersectionObserverEntry {
    pub fn intersection_ratio(&self) -> f64 {
        self.inner.get("intersectionRatio").as_::<f64>()
    }
}
impl IntersectionObserverEntry {
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }
}
