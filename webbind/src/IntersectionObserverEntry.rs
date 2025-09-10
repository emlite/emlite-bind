use super::*;

/// The IntersectionObserverEntry class.
/// [`IntersectionObserverEntry`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntersectionObserverEntry {
    inner: Any,
}

impl FromVal for IntersectionObserverEntry {
    fn from_val(v: &Any) -> Self {
        IntersectionObserverEntry {
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

impl core::ops::Deref for IntersectionObserverEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IntersectionObserverEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IntersectionObserverEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IntersectionObserverEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IntersectionObserverEntry> for Any {
    fn from(s: IntersectionObserverEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IntersectionObserverEntry> for Any {
    fn from(s: &IntersectionObserverEntry) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IntersectionObserverEntry);

impl IntersectionObserverEntry {
    /// The `new IntersectionObserverEntry(..)` constructor, creating a new IntersectionObserverEntry instance
    pub fn new(
        intersection_observer_entry_init: &IntersectionObserverEntryInit,
    ) -> IntersectionObserverEntry {
        Self {
            inner: Any::global("IntersectionObserverEntry")
                .new(&[intersection_observer_entry_init.into()])
                .as_::<Any>(),
        }
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `time` attribute.
    /// [`IntersectionObserverEntry.time`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/time)
    pub fn time(&self) -> Any {
        self.inner.get("time").as_::<Any>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `rootBounds` attribute.
    /// [`IntersectionObserverEntry.rootBounds`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/rootBounds)
    pub fn root_bounds(&self) -> DOMRectReadOnly {
        self.inner.get("rootBounds").as_::<DOMRectReadOnly>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `boundingClientRect` attribute.
    /// [`IntersectionObserverEntry.boundingClientRect`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/boundingClientRect)
    pub fn bounding_client_rect(&self) -> DOMRectReadOnly {
        self.inner
            .get("boundingClientRect")
            .as_::<DOMRectReadOnly>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `intersectionRect` attribute.
    /// [`IntersectionObserverEntry.intersectionRect`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRect)
    pub fn intersection_rect(&self) -> DOMRectReadOnly {
        self.inner.get("intersectionRect").as_::<DOMRectReadOnly>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `isIntersecting` attribute.
    /// [`IntersectionObserverEntry.isIntersecting`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isIntersecting)
    pub fn is_intersecting(&self) -> bool {
        self.inner.get("isIntersecting").as_::<bool>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `isVisible` attribute.
    /// [`IntersectionObserverEntry.isVisible`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isVisible)
    pub fn is_visible(&self) -> bool {
        self.inner.get("isVisible").as_::<bool>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `intersectionRatio` attribute.
    /// [`IntersectionObserverEntry.intersectionRatio`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio)
    pub fn intersection_ratio(&self) -> f64 {
        self.inner.get("intersectionRatio").as_::<f64>()
    }
}
impl IntersectionObserverEntry {
    /// Getter of the `target` attribute.
    /// [`IntersectionObserverEntry.target`](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/target)
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }
}
