use super::*;

/// The HTMLOListElement class.
/// [`HTMLOListElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOListElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOListElement {
    fn from_val(v: &Any) -> Self {
        HTMLOListElement {
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
impl core::ops::Deref for HTMLOListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLOListElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLOListElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLOListElement> for Any {
    fn from(s: HTMLOListElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLOListElement> for Any {
    fn from(s: &HTMLOListElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOListElement);

impl HTMLOListElement {
    /// The `new HTMLOListElement(..)` constructor, creating a new HTMLOListElement instance
    pub fn new() -> HTMLOListElement {
        Self {
            inner: Any::global("HTMLOListElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOListElement {
    /// Getter of the `reversed` attribute.
    /// [`HTMLOListElement.reversed`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)
    pub fn reversed(&self) -> bool {
        self.inner.get("reversed").as_::<bool>()
    }

    /// Setter of the `reversed` attribute.
    /// [`HTMLOListElement.reversed`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)
    pub fn set_reversed(&mut self, value: bool) {
        self.inner.set("reversed", value);
    }
}
impl HTMLOListElement {
    /// Getter of the `start` attribute.
    /// [`HTMLOListElement.start`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)
    pub fn start(&self) -> i32 {
        self.inner.get("start").as_::<i32>()
    }

    /// Setter of the `start` attribute.
    /// [`HTMLOListElement.start`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)
    pub fn set_start(&mut self, value: i32) {
        self.inner.set("start", value);
    }
}
impl HTMLOListElement {
    /// Getter of the `type` attribute.
    /// [`HTMLOListElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    /// Setter of the `type` attribute.
    /// [`HTMLOListElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)
    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl HTMLOListElement {
    /// Getter of the `compact` attribute.
    /// [`HTMLOListElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)
    pub fn compact(&self) -> bool {
        self.inner.get("compact").as_::<bool>()
    }

    /// Setter of the `compact` attribute.
    /// [`HTMLOListElement.compact`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)
    pub fn set_compact(&mut self, value: bool) {
        self.inner.set("compact", value);
    }
}
