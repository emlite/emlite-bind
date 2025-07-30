use super::*;

/// The HTMLQuoteElement class.
/// [`HTMLQuoteElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLQuoteElement {
    inner: HTMLElement,
}
impl FromVal for HTMLQuoteElement {
    fn from_val(v: &Any) -> Self {
        HTMLQuoteElement {
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
impl core::ops::Deref for HTMLQuoteElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLQuoteElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLQuoteElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLQuoteElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLQuoteElement> for Any {
    fn from(s: HTMLQuoteElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLQuoteElement> for Any {
    fn from(s: &HTMLQuoteElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLQuoteElement);

impl HTMLQuoteElement {
    /// The `new HTMLQuoteElement(..)` constructor, creating a new HTMLQuoteElement instance
    pub fn new() -> HTMLQuoteElement {
        Self {
            inner: Any::global("HTMLQuoteElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLQuoteElement {
    /// Getter of the `cite` attribute.
    /// [`HTMLQuoteElement.cite`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)
    pub fn cite(&self) -> JsString {
        self.inner.get("cite").as_::<JsString>()
    }

    /// Setter of the `cite` attribute.
    /// [`HTMLQuoteElement.cite`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)
    pub fn set_cite(&mut self, value: &JsString) {
        self.inner.set("cite", value);
    }
}
