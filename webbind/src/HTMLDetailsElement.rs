use super::*;

/// The HTMLDetailsElement class.
/// [`HTMLDetailsElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLDetailsElement {
    inner: HTMLElement,
}

impl FromVal for HTMLDetailsElement {
    fn from_val(v: &Any) -> Self {
        HTMLDetailsElement {
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

impl core::ops::Deref for HTMLDetailsElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLDetailsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLDetailsElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLDetailsElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLDetailsElement> for Any {
    fn from(s: HTMLDetailsElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLDetailsElement> for Any {
    fn from(s: &HTMLDetailsElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLDetailsElement);

impl HTMLDetailsElement {
    /// The `new HTMLDetailsElement(..)` constructor, creating a new HTMLDetailsElement instance
    pub fn new() -> HTMLDetailsElement {
        Self {
            inner: Any::global("HTMLDetailsElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLDetailsElement {
    /// Getter of the `name` attribute.
    /// [`HTMLDetailsElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLDetailsElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl HTMLDetailsElement {
    /// Getter of the `open` attribute.
    /// [`HTMLDetailsElement.open`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/open)
    pub fn open(&self) -> bool {
        self.inner.get("open").as_::<bool>()
    }

    /// Setter of the `open` attribute.
    /// [`HTMLDetailsElement.open`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/open)
    pub fn set_open(&mut self, value: bool) {
        self.inner.set("open", value);
    }
}
