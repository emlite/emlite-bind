use super::*;

/// The HTMLMarqueeElement class.
/// [`HTMLMarqueeElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMarqueeElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMarqueeElement {
    fn from_val(v: &Any) -> Self {
        HTMLMarqueeElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLMarqueeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMarqueeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLMarqueeElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLMarqueeElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLMarqueeElement> for Any {
    fn from(s: HTMLMarqueeElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLMarqueeElement> for Any {
    fn from(s: &HTMLMarqueeElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMarqueeElement);

impl HTMLMarqueeElement {
    /// The `new HTMLMarqueeElement(..)` constructor, creating a new HTMLMarqueeElement instance
    pub fn new() -> HTMLMarqueeElement {
        Self {
            inner: Any::global("HTMLMarqueeElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `behavior` attribute.
    /// [`HTMLMarqueeElement.behavior`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/behavior)
    pub fn behavior(&self) -> JsString {
        self.inner.get("behavior").as_::<JsString>()
    }

    /// Setter of the `behavior` attribute.
    /// [`HTMLMarqueeElement.behavior`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/behavior)
    pub fn set_behavior(&mut self, value: &JsString) {
        self.inner.set("behavior", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `bgColor` attribute.
    /// [`HTMLMarqueeElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/bgColor)
    pub fn bg_color(&self) -> JsString {
        self.inner.get("bgColor").as_::<JsString>()
    }

    /// Setter of the `bgColor` attribute.
    /// [`HTMLMarqueeElement.bgColor`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/bgColor)
    pub fn set_bg_color(&mut self, value: &JsString) {
        self.inner.set("bgColor", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `direction` attribute.
    /// [`HTMLMarqueeElement.direction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/direction)
    pub fn direction(&self) -> JsString {
        self.inner.get("direction").as_::<JsString>()
    }

    /// Setter of the `direction` attribute.
    /// [`HTMLMarqueeElement.direction`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/direction)
    pub fn set_direction(&mut self, value: &JsString) {
        self.inner.set("direction", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `height` attribute.
    /// [`HTMLMarqueeElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/height)
    pub fn height(&self) -> JsString {
        self.inner.get("height").as_::<JsString>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLMarqueeElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/height)
    pub fn set_height(&mut self, value: &JsString) {
        self.inner.set("height", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `hspace` attribute.
    /// [`HTMLMarqueeElement.hspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/hspace)
    pub fn hspace(&self) -> u32 {
        self.inner.get("hspace").as_::<u32>()
    }

    /// Setter of the `hspace` attribute.
    /// [`HTMLMarqueeElement.hspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/hspace)
    pub fn set_hspace(&mut self, value: u32) {
        self.inner.set("hspace", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `loop` attribute.
    /// [`HTMLMarqueeElement.loop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/loop)
    pub fn loop_(&self) -> i32 {
        self.inner.get("loop").as_::<i32>()
    }

    /// Setter of the `loop` attribute.
    /// [`HTMLMarqueeElement.loop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/loop)
    pub fn set_loop_(&mut self, value: i32) {
        self.inner.set("loop", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `scrollAmount` attribute.
    /// [`HTMLMarqueeElement.scrollAmount`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/scrollAmount)
    pub fn scroll_amount(&self) -> u32 {
        self.inner.get("scrollAmount").as_::<u32>()
    }

    /// Setter of the `scrollAmount` attribute.
    /// [`HTMLMarqueeElement.scrollAmount`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/scrollAmount)
    pub fn set_scroll_amount(&mut self, value: u32) {
        self.inner.set("scrollAmount", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `scrollDelay` attribute.
    /// [`HTMLMarqueeElement.scrollDelay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/scrollDelay)
    pub fn scroll_delay(&self) -> u32 {
        self.inner.get("scrollDelay").as_::<u32>()
    }

    /// Setter of the `scrollDelay` attribute.
    /// [`HTMLMarqueeElement.scrollDelay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/scrollDelay)
    pub fn set_scroll_delay(&mut self, value: u32) {
        self.inner.set("scrollDelay", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `trueSpeed` attribute.
    /// [`HTMLMarqueeElement.trueSpeed`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/trueSpeed)
    pub fn true_speed(&self) -> bool {
        self.inner.get("trueSpeed").as_::<bool>()
    }

    /// Setter of the `trueSpeed` attribute.
    /// [`HTMLMarqueeElement.trueSpeed`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/trueSpeed)
    pub fn set_true_speed(&mut self, value: bool) {
        self.inner.set("trueSpeed", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `vspace` attribute.
    /// [`HTMLMarqueeElement.vspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/vspace)
    pub fn vspace(&self) -> u32 {
        self.inner.get("vspace").as_::<u32>()
    }

    /// Setter of the `vspace` attribute.
    /// [`HTMLMarqueeElement.vspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/vspace)
    pub fn set_vspace(&mut self, value: u32) {
        self.inner.set("vspace", value);
    }
}
impl HTMLMarqueeElement {
    /// Getter of the `width` attribute.
    /// [`HTMLMarqueeElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/width)
    pub fn width(&self) -> JsString {
        self.inner.get("width").as_::<JsString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLMarqueeElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/width)
    pub fn set_width(&mut self, value: &JsString) {
        self.inner.set("width", value);
    }
}
impl HTMLMarqueeElement {
    /// The start method.
    /// [`HTMLMarqueeElement.start`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/start)
    pub fn start(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
}
impl HTMLMarqueeElement {
    /// The stop method.
    /// [`HTMLMarqueeElement.stop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMarqueeElement/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
