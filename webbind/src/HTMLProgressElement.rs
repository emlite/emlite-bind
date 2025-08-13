use super::*;




/// The HTMLProgressElement class.
/// [`HTMLProgressElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLProgressElement {
    inner: HTMLElement,
}

impl FromVal for HTMLProgressElement {
    fn from_val(v: &Any) -> Self {
        HTMLProgressElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLProgressElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLProgressElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLProgressElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLProgressElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLProgressElement> for Any {
    fn from(s: HTMLProgressElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLProgressElement> for Any {
    fn from(s: &HTMLProgressElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLProgressElement);



impl HTMLProgressElement {
    /// The `new HTMLProgressElement(..)` constructor, creating a new HTMLProgressElement instance
    pub fn new() -> HTMLProgressElement {
        Self {
            inner: Any::global("HTMLProgressElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLProgressElement {
    /// Getter of the `value` attribute.
    /// [`HTMLProgressElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLProgressElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)
    pub fn set_value(&mut self, value: f64) {
        self.inner.set("value", value);
    }
}
impl HTMLProgressElement {
    /// Getter of the `max` attribute.
    /// [`HTMLProgressElement.max`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }

    /// Setter of the `max` attribute.
    /// [`HTMLProgressElement.max`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)
    pub fn set_max(&mut self, value: f64) {
        self.inner.set("max", value);
    }
}
impl HTMLProgressElement {
    /// Getter of the `position` attribute.
    /// [`HTMLProgressElement.position`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/position)
    pub fn position(&self) -> f64 {
        self.inner.get("position").as_::<f64>()
    }

}
impl HTMLProgressElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLProgressElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }

}
