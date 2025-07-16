use super::*;

/// The ResizeObserverEntry class.
/// [`ResizeObserverEntry`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResizeObserverEntry {
    inner: Any,
}
impl FromVal for ResizeObserverEntry {
    fn from_val(v: &Any) -> Self {
        ResizeObserverEntry {
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
impl core::ops::Deref for ResizeObserverEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ResizeObserverEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ResizeObserverEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ResizeObserverEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ResizeObserverEntry> for Any {
    fn from(s: ResizeObserverEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ResizeObserverEntry> for Any {
    fn from(s: &ResizeObserverEntry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ResizeObserverEntry);

impl ResizeObserverEntry {
    /// Getter of the `target` attribute.
    /// [`ResizeObserverEntry.target`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/target)
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }
}
impl ResizeObserverEntry {
    /// Getter of the `contentRect` attribute.
    /// [`ResizeObserverEntry.contentRect`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentRect)
    pub fn content_rect(&self) -> DOMRectReadOnly {
        self.inner.get("contentRect").as_::<DOMRectReadOnly>()
    }
}
impl ResizeObserverEntry {
    /// Getter of the `borderBoxSize` attribute.
    /// [`ResizeObserverEntry.borderBoxSize`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/borderBoxSize)
    pub fn border_box_size(&self) -> FrozenArray<ResizeObserverSize> {
        self.inner
            .get("borderBoxSize")
            .as_::<FrozenArray<ResizeObserverSize>>()
    }
}
impl ResizeObserverEntry {
    /// Getter of the `contentBoxSize` attribute.
    /// [`ResizeObserverEntry.contentBoxSize`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentBoxSize)
    pub fn content_box_size(&self) -> FrozenArray<ResizeObserverSize> {
        self.inner
            .get("contentBoxSize")
            .as_::<FrozenArray<ResizeObserverSize>>()
    }
}
impl ResizeObserverEntry {
    /// Getter of the `devicePixelContentBoxSize` attribute.
    /// [`ResizeObserverEntry.devicePixelContentBoxSize`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/devicePixelContentBoxSize)
    pub fn device_pixel_content_box_size(&self) -> FrozenArray<ResizeObserverSize> {
        self.inner
            .get("devicePixelContentBoxSize")
            .as_::<FrozenArray<ResizeObserverSize>>()
    }
}
