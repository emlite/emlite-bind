use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResizeObserverEntry {
    inner: emlite::Val,
}
impl FromVal for ResizeObserverEntry {
    fn from_val(v: &emlite::Val) -> Self {
        ResizeObserverEntry {
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
impl core::ops::Deref for ResizeObserverEntry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ResizeObserverEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ResizeObserverEntry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ResizeObserverEntry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ResizeObserverEntry> for emlite::Val {
    fn from(s: ResizeObserverEntry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ResizeObserverEntry);

impl ResizeObserverEntry {
    pub fn target(&self) -> Element {
        self.inner.get("target").as_::<Element>()
    }
}
impl ResizeObserverEntry {
    pub fn content_rect(&self) -> DOMRectReadOnly {
        self.inner.get("contentRect").as_::<DOMRectReadOnly>()
    }
}
impl ResizeObserverEntry {
    pub fn border_box_size(&self) -> jsbind::FrozenArray<ResizeObserverSize> {
        self.inner
            .get("borderBoxSize")
            .as_::<jsbind::FrozenArray<ResizeObserverSize>>()
    }
}
impl ResizeObserverEntry {
    pub fn content_box_size(&self) -> jsbind::FrozenArray<ResizeObserverSize> {
        self.inner
            .get("contentBoxSize")
            .as_::<jsbind::FrozenArray<ResizeObserverSize>>()
    }
}
impl ResizeObserverEntry {
    pub fn device_pixel_content_box_size(&self) -> jsbind::FrozenArray<ResizeObserverSize> {
        self.inner
            .get("devicePixelContentBoxSize")
            .as_::<jsbind::FrozenArray<ResizeObserverSize>>()
    }
}
