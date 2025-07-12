use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for IntersectionObserverEntry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IntersectionObserverEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IntersectionObserverEntry> for emlite::Val {
    fn from(s: IntersectionObserverEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IntersectionObserverEntry {
    pub fn new(intersection_observer_entry_init: jsbind::Any) -> IntersectionObserverEntry {
        Self {
            inner: emlite::Val::global("IntersectionObserverEntry")
                .new(&[intersection_observer_entry_init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl IntersectionObserverEntry {
    pub fn time(&self) -> jsbind::Any {
        self.inner.get("time").as_::<jsbind::Any>()
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
