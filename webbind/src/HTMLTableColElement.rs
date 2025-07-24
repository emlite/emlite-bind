use super::*;

/// The HTMLTableColElement class.
/// [`HTMLTableColElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTableColElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableColElement {
    fn from_val(v: &Any) -> Self {
        HTMLTableColElement {
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
impl core::ops::Deref for HTMLTableColElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableColElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLTableColElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLTableColElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLTableColElement> for Any {
    fn from(s: HTMLTableColElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLTableColElement> for Any {
    fn from(s: &HTMLTableColElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableColElement);

impl HTMLTableColElement {
    /// The `new HTMLTableColElement(..)` constructor, creating a new HTMLTableColElement instance
    pub fn new() -> HTMLTableColElement {
        Self {
            inner: Any::global("HTMLTableColElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableColElement {
    /// Getter of the `span` attribute.
    /// [`HTMLTableColElement.span`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)
    pub fn span(&self) -> u32 {
        self.inner.get("span").as_::<u32>()
    }

    /// Setter of the `span` attribute.
    /// [`HTMLTableColElement.span`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)
    pub fn set_span(&mut self, value: u32) {
        self.inner.set("span", value);
    }
}
impl HTMLTableColElement {
    /// Getter of the `align` attribute.
    /// [`HTMLTableColElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLTableColElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)
    pub fn set_align(&mut self, value: &DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableColElement {
    /// Getter of the `ch` attribute.
    /// [`HTMLTableColElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)
    pub fn ch(&self) -> DOMString {
        self.inner.get("ch").as_::<DOMString>()
    }

    /// Setter of the `ch` attribute.
    /// [`HTMLTableColElement.ch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)
    pub fn set_ch(&mut self, value: &DOMString) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableColElement {
    /// Getter of the `chOff` attribute.
    /// [`HTMLTableColElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)
    pub fn ch_off(&self) -> DOMString {
        self.inner.get("chOff").as_::<DOMString>()
    }

    /// Setter of the `chOff` attribute.
    /// [`HTMLTableColElement.chOff`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)
    pub fn set_ch_off(&mut self, value: &DOMString) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableColElement {
    /// Getter of the `vAlign` attribute.
    /// [`HTMLTableColElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)
    pub fn v_align(&self) -> DOMString {
        self.inner.get("vAlign").as_::<DOMString>()
    }

    /// Setter of the `vAlign` attribute.
    /// [`HTMLTableColElement.vAlign`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)
    pub fn set_v_align(&mut self, value: &DOMString) {
        self.inner.set("vAlign", value);
    }
}
impl HTMLTableColElement {
    /// Getter of the `width` attribute.
    /// [`HTMLTableColElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)
    pub fn width(&self) -> DOMString {
        self.inner.get("width").as_::<DOMString>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLTableColElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)
    pub fn set_width(&mut self, value: &DOMString) {
        self.inner.set("width", value);
    }
}
