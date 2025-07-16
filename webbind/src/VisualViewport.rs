use super::*;

/// The VisualViewport class.
/// [`VisualViewport`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VisualViewport {
    inner: EventTarget,
}
impl FromVal for VisualViewport {
    fn from_val(v: &Any) -> Self {
        VisualViewport {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VisualViewport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VisualViewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VisualViewport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VisualViewport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VisualViewport> for Any {
    fn from(s: VisualViewport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VisualViewport> for Any {
    fn from(s: &VisualViewport) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VisualViewport);

impl VisualViewport {
    /// Getter of the `offsetLeft` attribute.
    /// [`VisualViewport.offsetLeft`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/offsetLeft)
    pub fn offset_left(&self) -> f64 {
        self.inner.get("offsetLeft").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `offsetTop` attribute.
    /// [`VisualViewport.offsetTop`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/offsetTop)
    pub fn offset_top(&self) -> f64 {
        self.inner.get("offsetTop").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `pageLeft` attribute.
    /// [`VisualViewport.pageLeft`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/pageLeft)
    pub fn page_left(&self) -> f64 {
        self.inner.get("pageLeft").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `pageTop` attribute.
    /// [`VisualViewport.pageTop`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/pageTop)
    pub fn page_top(&self) -> f64 {
        self.inner.get("pageTop").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `width` attribute.
    /// [`VisualViewport.width`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `height` attribute.
    /// [`VisualViewport.height`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/height)
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `scale` attribute.
    /// [`VisualViewport.scale`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/scale)
    pub fn scale(&self) -> f64 {
        self.inner.get("scale").as_::<f64>()
    }
}
impl VisualViewport {
    /// Getter of the `onresize` attribute.
    /// [`VisualViewport.onresize`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onresize)
    pub fn onresize(&self) -> Any {
        self.inner.get("onresize").as_::<Any>()
    }

    /// Setter of the `onresize` attribute.
    /// [`VisualViewport.onresize`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onresize)
    pub fn set_onresize(&mut self, value: &Any) {
        self.inner.set("onresize", value);
    }
}
impl VisualViewport {
    /// Getter of the `onscroll` attribute.
    /// [`VisualViewport.onscroll`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscroll)
    pub fn onscroll(&self) -> Any {
        self.inner.get("onscroll").as_::<Any>()
    }

    /// Setter of the `onscroll` attribute.
    /// [`VisualViewport.onscroll`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscroll)
    pub fn set_onscroll(&mut self, value: &Any) {
        self.inner.set("onscroll", value);
    }
}
impl VisualViewport {
    /// Getter of the `onscrollend` attribute.
    /// [`VisualViewport.onscrollend`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscrollend)
    pub fn onscrollend(&self) -> Any {
        self.inner.get("onscrollend").as_::<Any>()
    }

    /// Setter of the `onscrollend` attribute.
    /// [`VisualViewport.onscrollend`](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscrollend)
    pub fn set_onscrollend(&mut self, value: &Any) {
        self.inner.set("onscrollend", value);
    }
}
