use super::*;

/// The LargestContentfulPaint class.
/// [`LargestContentfulPaint`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LargestContentfulPaint {
    inner: PerformanceEntry,
}
impl FromVal for LargestContentfulPaint {
    fn from_val(v: &Any) -> Self {
        LargestContentfulPaint {
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
impl core::ops::Deref for LargestContentfulPaint {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LargestContentfulPaint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LargestContentfulPaint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LargestContentfulPaint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LargestContentfulPaint> for Any {
    fn from(s: LargestContentfulPaint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LargestContentfulPaint> for Any {
    fn from(s: &LargestContentfulPaint) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LargestContentfulPaint);

impl LargestContentfulPaint {
    /// Getter of the `loadTime` attribute.
    /// [`LargestContentfulPaint.loadTime`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/loadTime)
    pub fn load_time(&self) -> Any {
        self.inner.get("loadTime").as_::<Any>()
    }
}
impl LargestContentfulPaint {
    /// Getter of the `size` attribute.
    /// [`LargestContentfulPaint.size`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl LargestContentfulPaint {
    /// Getter of the `id` attribute.
    /// [`LargestContentfulPaint.id`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/id)
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
impl LargestContentfulPaint {
    /// Getter of the `url` attribute.
    /// [`LargestContentfulPaint.url`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/url)
    pub fn url(&self) -> DOMString {
        self.inner.get("url").as_::<DOMString>()
    }
}
impl LargestContentfulPaint {
    /// Getter of the `element` attribute.
    /// [`LargestContentfulPaint.element`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/element)
    pub fn element(&self) -> Element {
        self.inner.get("element").as_::<Element>()
    }
}
impl LargestContentfulPaint {
    /// The toJSON method.
    /// [`LargestContentfulPaint.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl LargestContentfulPaint {
    /// Getter of the `paintTime` attribute.
    /// [`LargestContentfulPaint.paintTime`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/paintTime)
    pub fn paint_time(&self) -> Any {
        self.inner.get("paintTime").as_::<Any>()
    }
}
impl LargestContentfulPaint {
    /// Getter of the `presentationTime` attribute.
    /// [`LargestContentfulPaint.presentationTime`](https://developer.mozilla.org/en-US/docs/Web/API/LargestContentfulPaint/presentationTime)
    pub fn presentation_time(&self) -> Any {
        self.inner.get("presentationTime").as_::<Any>()
    }
}
