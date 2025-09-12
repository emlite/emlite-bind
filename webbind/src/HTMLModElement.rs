use super::*;

/// The HTMLModElement class.
/// [`HTMLModElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLModElement {
    inner: HTMLElement,
}

impl FromVal for HTMLModElement {
    fn from_val(v: &Any) -> Self {
        HTMLModElement {
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

impl core::ops::Deref for HTMLModElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLModElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLModElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLModElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLModElement> for Any {
    fn from(s: HTMLModElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLModElement> for Any {
    fn from(s: &HTMLModElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLModElement);

impl HTMLModElement {
    /// Getter of the `cite` attribute.
    /// [`HTMLModElement.cite`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)
    pub fn cite(&self) -> JsString {
        self.inner.get("cite").as_::<JsString>()
    }

    /// Setter of the `cite` attribute.
    /// [`HTMLModElement.cite`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)
    pub fn set_cite(&mut self, value: &JsString) {
        self.inner.set("cite", value);
    }
}
impl HTMLModElement {
    /// Getter of the `dateTime` attribute.
    /// [`HTMLModElement.dateTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)
    pub fn date_time(&self) -> JsString {
        self.inner.get("dateTime").as_::<JsString>()
    }

    /// Setter of the `dateTime` attribute.
    /// [`HTMLModElement.dateTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)
    pub fn set_date_time(&mut self, value: &JsString) {
        self.inner.set("dateTime", value);
    }
}

impl HTMLModElement {
    /// The `new HTMLModElement(..)` constructor, creating a new HTMLModElement instance
    pub fn new() -> HTMLModElement {
        Self {
            inner: Any::global("HTMLModElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
