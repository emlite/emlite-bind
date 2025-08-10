use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntersectionObserverEntryInit {
    inner: Any,
}
impl FromVal for IntersectionObserverEntryInit {
    fn from_val(v: &Any) -> Self {
        IntersectionObserverEntryInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IntersectionObserverEntryInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IntersectionObserverEntryInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IntersectionObserverEntryInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IntersectionObserverEntryInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IntersectionObserverEntryInit> for Any {
    fn from(s: IntersectionObserverEntryInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IntersectionObserverEntryInit> for Any {
    fn from(s: &IntersectionObserverEntryInit) -> Any {
        s.inner.clone()
    }
}

impl IntersectionObserverEntryInit {
    pub fn time(&self) -> Any {
        self.inner.get("time").as_::<Any>()
    }

    pub fn set_time(&mut self, value: &Any) {
        self.inner.set("time", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn root_bounds(&self) -> DOMRectInit {
        self.inner.get("rootBounds").as_::<DOMRectInit>()
    }

    pub fn set_root_bounds(&mut self, value: &DOMRectInit) {
        self.inner.set("rootBounds", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn bounding_client_rect(&self) -> DOMRectInit {
        self.inner.get("boundingClientRect").as_::<DOMRectInit>()
    }

    pub fn set_bounding_client_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("boundingClientRect", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn intersection_rect(&self) -> DOMRectInit {
        self.inner.get("intersectionRect").as_::<DOMRectInit>()
    }

    pub fn set_intersection_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("intersectionRect", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn is_intersecting(&self) -> bool {
        self.inner.get("isIntersecting").as_::<bool>()
    }

    pub fn set_is_intersecting(&mut self, value: bool) {
        self.inner.set("isIntersecting", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn is_visible(&self) -> bool {
        self.inner.get("isVisible").as_::<bool>()
    }

    pub fn set_is_visible(&mut self, value: bool) {
        self.inner.set("isVisible", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn intersection_ratio(&self) -> f64 {
        self.inner.get("intersectionRatio").as_::<f64>()
    }

    pub fn set_intersection_ratio(&mut self, value: f64) {
        self.inner.set("intersectionRatio", value);
    }
}
impl IntersectionObserverEntryInit {
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }

    pub fn set_target(&mut self, value: &Element) {
        self.inner.set("target", value);
    }
}
