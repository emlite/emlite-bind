use super::*;

/// The SVGAnimationElement class.
/// [`SVGAnimationElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAnimationElement {
    inner: SVGElement,
}

impl FromVal for SVGAnimationElement {
    fn from_val(v: &Any) -> Self {
        SVGAnimationElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGAnimationElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGAnimationElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGAnimationElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGAnimationElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGAnimationElement> for Any {
    fn from(s: SVGAnimationElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGAnimationElement> for Any {
    fn from(s: &SVGAnimationElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGAnimationElement);

impl SVGAnimationElement {
    /// Getter of the `targetElement` attribute.
    /// [`SVGAnimationElement.targetElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/targetElement)
    pub fn target_element(&self) -> SVGElement {
        self.inner.get("targetElement").as_::<SVGElement>()
    }
}
impl SVGAnimationElement {
    /// Getter of the `onbegin` attribute.
    /// [`SVGAnimationElement.onbegin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/onbegin)
    pub fn onbegin(&self) -> Any {
        self.inner.get("onbegin").as_::<Any>()
    }

    /// Setter of the `onbegin` attribute.
    /// [`SVGAnimationElement.onbegin`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/onbegin)
    pub fn set_onbegin(&mut self, value: &Any) {
        self.inner.set("onbegin", value);
    }
}
impl SVGAnimationElement {
    /// Getter of the `onend` attribute.
    /// [`SVGAnimationElement.onend`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/onend)
    pub fn onend(&self) -> Any {
        self.inner.get("onend").as_::<Any>()
    }

    /// Setter of the `onend` attribute.
    /// [`SVGAnimationElement.onend`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/onend)
    pub fn set_onend(&mut self, value: &Any) {
        self.inner.set("onend", value);
    }
}
impl SVGAnimationElement {
    /// Getter of the `onrepeat` attribute.
    /// [`SVGAnimationElement.onrepeat`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/onrepeat)
    pub fn onrepeat(&self) -> Any {
        self.inner.get("onrepeat").as_::<Any>()
    }

    /// Setter of the `onrepeat` attribute.
    /// [`SVGAnimationElement.onrepeat`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/onrepeat)
    pub fn set_onrepeat(&mut self, value: &Any) {
        self.inner.set("onrepeat", value);
    }
}
impl SVGAnimationElement {
    /// The getStartTime method.
    /// [`SVGAnimationElement.getStartTime`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getStartTime)
    pub fn get_start_time(&self) -> f32 {
        self.inner.call("getStartTime", &[]).as_::<f32>()
    }
}
impl SVGAnimationElement {
    /// The getCurrentTime method.
    /// [`SVGAnimationElement.getCurrentTime`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getCurrentTime)
    pub fn get_current_time(&self) -> f32 {
        self.inner.call("getCurrentTime", &[]).as_::<f32>()
    }
}
impl SVGAnimationElement {
    /// The getSimpleDuration method.
    /// [`SVGAnimationElement.getSimpleDuration`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getSimpleDuration)
    pub fn get_simple_duration(&self) -> f32 {
        self.inner.call("getSimpleDuration", &[]).as_::<f32>()
    }
}
impl SVGAnimationElement {
    /// The beginElement method.
    /// [`SVGAnimationElement.beginElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElement)
    pub fn begin_element(&self) -> Undefined {
        self.inner.call("beginElement", &[]).as_::<Undefined>()
    }
}
impl SVGAnimationElement {
    /// The beginElementAt method.
    /// [`SVGAnimationElement.beginElementAt`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElementAt)
    pub fn begin_element_at(&self, offset: f32) -> Undefined {
        self.inner
            .call("beginElementAt", &[offset.into()])
            .as_::<Undefined>()
    }
}
impl SVGAnimationElement {
    /// The endElement method.
    /// [`SVGAnimationElement.endElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElement)
    pub fn end_element(&self) -> Undefined {
        self.inner.call("endElement", &[]).as_::<Undefined>()
    }
}
impl SVGAnimationElement {
    /// The endElementAt method.
    /// [`SVGAnimationElement.endElementAt`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElementAt)
    pub fn end_element_at(&self, offset: f32) -> Undefined {
        self.inner
            .call("endElementAt", &[offset.into()])
            .as_::<Undefined>()
    }
}
impl SVGAnimationElement {
    /// Getter of the `requiredExtensions` attribute.
    /// [`SVGAnimationElement.requiredExtensions`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredExtensions)
    pub fn required_extensions(&self) -> SVGStringList {
        self.inner.get("requiredExtensions").as_::<SVGStringList>()
    }
}
impl SVGAnimationElement {
    /// Getter of the `systemLanguage` attribute.
    /// [`SVGAnimationElement.systemLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/systemLanguage)
    pub fn system_language(&self) -> SVGStringList {
        self.inner.get("systemLanguage").as_::<SVGStringList>()
    }
}
