use super::*;

/// The PerformanceElementTiming class.
/// [`PerformanceElementTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceElementTiming {
    inner: PerformanceEntry,
}

impl FromVal for PerformanceElementTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceElementTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PerformanceElementTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceElementTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceElementTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceElementTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformanceElementTiming> for Any {
    fn from(s: PerformanceElementTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceElementTiming> for Any {
    fn from(s: &PerformanceElementTiming) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceElementTiming);

impl PerformanceElementTiming {
    /// Getter of the `renderTime` attribute.
    /// [`PerformanceElementTiming.renderTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/renderTime)
    pub fn render_time(&self) -> Any {
        self.inner.get("renderTime").as_::<Any>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `loadTime` attribute.
    /// [`PerformanceElementTiming.loadTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/loadTime)
    pub fn load_time(&self) -> Any {
        self.inner.get("loadTime").as_::<Any>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `intersectionRect` attribute.
    /// [`PerformanceElementTiming.intersectionRect`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/intersectionRect)
    pub fn intersection_rect(&self) -> DOMRectReadOnly {
        self.inner.get("intersectionRect").as_::<DOMRectReadOnly>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `identifier` attribute.
    /// [`PerformanceElementTiming.identifier`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/identifier)
    pub fn identifier(&self) -> JsString {
        self.inner.get("identifier").as_::<JsString>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `naturalWidth` attribute.
    /// [`PerformanceElementTiming.naturalWidth`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/naturalWidth)
    pub fn natural_width(&self) -> u32 {
        self.inner.get("naturalWidth").as_::<u32>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `naturalHeight` attribute.
    /// [`PerformanceElementTiming.naturalHeight`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/naturalHeight)
    pub fn natural_height(&self) -> u32 {
        self.inner.get("naturalHeight").as_::<u32>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `id` attribute.
    /// [`PerformanceElementTiming.id`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `element` attribute.
    /// [`PerformanceElementTiming.element`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/element)
    pub fn element(&self) -> Element {
        self.inner.get("element").as_::<Element>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `url` attribute.
    /// [`PerformanceElementTiming.url`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `paintTime` attribute.
    /// [`PerformanceElementTiming.paintTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/paintTime)
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }
}
impl PerformanceElementTiming {
    /// Getter of the `presentationTime` attribute.
    /// [`PerformanceElementTiming.presentationTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/presentationTime)
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }
}
impl PerformanceElementTiming {
    /// The toJSON method.
    /// [`PerformanceElementTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceElementTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
