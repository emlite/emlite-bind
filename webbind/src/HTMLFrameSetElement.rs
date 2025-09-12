use super::*;

/// The HTMLFrameSetElement class.
/// [`HTMLFrameSetElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFrameSetElement {
    inner: HTMLElement,
}

impl FromVal for HTMLFrameSetElement {
    fn from_val(v: &Any) -> Self {
        HTMLFrameSetElement {
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

impl core::ops::Deref for HTMLFrameSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLFrameSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLFrameSetElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLFrameSetElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLFrameSetElement> for Any {
    fn from(s: HTMLFrameSetElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLFrameSetElement> for Any {
    fn from(s: &HTMLFrameSetElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLFrameSetElement);

impl HTMLFrameSetElement {
    /// Getter of the `cols` attribute.
    /// [`HTMLFrameSetElement.cols`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)
    pub fn cols(&self) -> JsString {
        self.inner.get("cols").as_::<JsString>()
    }

    /// Setter of the `cols` attribute.
    /// [`HTMLFrameSetElement.cols`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)
    pub fn set_cols(&mut self, value: &JsString) {
        self.inner.set("cols", value);
    }
}
impl HTMLFrameSetElement {
    /// Getter of the `rows` attribute.
    /// [`HTMLFrameSetElement.rows`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)
    pub fn rows(&self) -> JsString {
        self.inner.get("rows").as_::<JsString>()
    }

    /// Setter of the `rows` attribute.
    /// [`HTMLFrameSetElement.rows`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)
    pub fn set_rows(&mut self, value: &JsString) {
        self.inner.set("rows", value);
    }
}
impl HTMLFrameSetElement {
    /// Getter of the `onportalactivate` attribute.
    /// [`HTMLFrameSetElement.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onportalactivate)
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    /// Setter of the `onportalactivate` attribute.
    /// [`HTMLFrameSetElement.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onportalactivate)
    pub fn set_onportalactivate(&mut self, value: &Any) {
        self.inner.set("onportalactivate", value);
    }
}

impl HTMLFrameSetElement {
    /// The `new HTMLFrameSetElement(..)` constructor, creating a new HTMLFrameSetElement instance
    pub fn new() -> HTMLFrameSetElement {
        Self {
            inner: Any::global("HTMLFrameSetElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
