use super::*;




/// The HTMLMeterElement class.
/// [`HTMLMeterElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMeterElement {
    inner: HTMLElement,
}

impl FromVal for HTMLMeterElement {
    fn from_val(v: &Any) -> Self {
        HTMLMeterElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLMeterElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLMeterElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLMeterElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLMeterElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLMeterElement> for Any {
    fn from(s: HTMLMeterElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLMeterElement> for Any {
    fn from(s: &HTMLMeterElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLMeterElement);



impl HTMLMeterElement {
    /// The `new HTMLMeterElement(..)` constructor, creating a new HTMLMeterElement instance
    pub fn new() -> HTMLMeterElement {
        Self {
            inner: Any::global("HTMLMeterElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLMeterElement {
    /// Getter of the `value` attribute.
    /// [`HTMLMeterElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }

    /// Setter of the `value` attribute.
    /// [`HTMLMeterElement.value`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)
    pub fn set_value(&mut self, value: f64) {
        self.inner.set("value", value);
    }
}
impl HTMLMeterElement {
    /// Getter of the `min` attribute.
    /// [`HTMLMeterElement.min`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)
    pub fn min(&self) -> f64 {
        self.inner.get("min").as_::<f64>()
    }

    /// Setter of the `min` attribute.
    /// [`HTMLMeterElement.min`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)
    pub fn set_min(&mut self, value: f64) {
        self.inner.set("min", value);
    }
}
impl HTMLMeterElement {
    /// Getter of the `max` attribute.
    /// [`HTMLMeterElement.max`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }

    /// Setter of the `max` attribute.
    /// [`HTMLMeterElement.max`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)
    pub fn set_max(&mut self, value: f64) {
        self.inner.set("max", value);
    }
}
impl HTMLMeterElement {
    /// Getter of the `low` attribute.
    /// [`HTMLMeterElement.low`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)
    pub fn low(&self) -> f64 {
        self.inner.get("low").as_::<f64>()
    }

    /// Setter of the `low` attribute.
    /// [`HTMLMeterElement.low`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)
    pub fn set_low(&mut self, value: f64) {
        self.inner.set("low", value);
    }
}
impl HTMLMeterElement {
    /// Getter of the `high` attribute.
    /// [`HTMLMeterElement.high`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)
    pub fn high(&self) -> f64 {
        self.inner.get("high").as_::<f64>()
    }

    /// Setter of the `high` attribute.
    /// [`HTMLMeterElement.high`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)
    pub fn set_high(&mut self, value: f64) {
        self.inner.set("high", value);
    }
}
impl HTMLMeterElement {
    /// Getter of the `optimum` attribute.
    /// [`HTMLMeterElement.optimum`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)
    pub fn optimum(&self) -> f64 {
        self.inner.get("optimum").as_::<f64>()
    }

    /// Setter of the `optimum` attribute.
    /// [`HTMLMeterElement.optimum`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)
    pub fn set_optimum(&mut self, value: f64) {
        self.inner.set("optimum", value);
    }
}
impl HTMLMeterElement {
    /// Getter of the `labels` attribute.
    /// [`HTMLMeterElement.labels`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/labels)
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }

}
