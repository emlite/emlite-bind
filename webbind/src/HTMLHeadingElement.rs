use super::*;

/// The HTMLHeadingElement class.
/// [`HTMLHeadingElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLHeadingElement {
    inner: HTMLElement,
}

impl FromVal for HTMLHeadingElement {
    fn from_val(v: &Any) -> Self {
        HTMLHeadingElement {
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

impl core::ops::Deref for HTMLHeadingElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLHeadingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLHeadingElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLHeadingElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLHeadingElement> for Any {
    fn from(s: HTMLHeadingElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLHeadingElement> for Any {
    fn from(s: &HTMLHeadingElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLHeadingElement);

impl HTMLHeadingElement {
    /// Getter of the `align` attribute.
    /// [`HTMLHeadingElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)
    pub fn align(&self) -> JsString {
        self.inner.get("align").as_::<JsString>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLHeadingElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHeadingElement/align)
    pub fn set_align(&mut self, value: &JsString) {
        self.inner.set("align", value);
    }
}

impl HTMLHeadingElement {
    /// The `new HTMLHeadingElement(..)` constructor, creating a new HTMLHeadingElement instance
    pub fn new() -> HTMLHeadingElement {
        Self {
            inner: Any::global("HTMLHeadingElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
