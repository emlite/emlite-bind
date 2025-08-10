use super::*;

/// The HTMLPreElement class.
/// [`HTMLPreElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLPreElement {
    inner: HTMLElement,
}

impl FromVal for HTMLPreElement {
    fn from_val(v: &Any) -> Self {
        HTMLPreElement {
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

impl core::ops::Deref for HTMLPreElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLPreElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLPreElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLPreElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLPreElement> for Any {
    fn from(s: HTMLPreElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLPreElement> for Any {
    fn from(s: &HTMLPreElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLPreElement);

impl HTMLPreElement {
    /// The `new HTMLPreElement(..)` constructor, creating a new HTMLPreElement instance
    pub fn new() -> HTMLPreElement {
        Self {
            inner: Any::global("HTMLPreElement").new(&[]).as_::<HTMLElement>(),
        }
    }
}
impl HTMLPreElement {
    /// Getter of the `width` attribute.
    /// [`HTMLPreElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement/width)
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLPreElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement/width)
    pub fn set_width(&mut self, value: i32) {
        self.inner.set("width", value);
    }
}
