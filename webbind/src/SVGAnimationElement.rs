use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SVGAnimationElement {
    inner: SVGElement,
}
impl FromVal for SVGAnimationElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAnimationElement {
            inner: SVGElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<SVGAnimationElement> for emlite::Val {
    fn from(s: SVGAnimationElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAnimationElement {
    pub fn target_element(&self) -> SVGElement {
        self.inner.get("targetElement").as_::<SVGElement>()
    }
}
impl SVGAnimationElement {
    pub fn onbegin(&self) -> jsbind::Any {
        self.inner.get("onbegin").as_::<jsbind::Any>()
    }

    pub fn set_onbegin(&mut self, value: jsbind::Any) {
        self.inner.set("onbegin", value);
    }
}
impl SVGAnimationElement {
    pub fn onend(&self) -> jsbind::Any {
        self.inner.get("onend").as_::<jsbind::Any>()
    }

    pub fn set_onend(&mut self, value: jsbind::Any) {
        self.inner.set("onend", value);
    }
}
impl SVGAnimationElement {
    pub fn onrepeat(&self) -> jsbind::Any {
        self.inner.get("onrepeat").as_::<jsbind::Any>()
    }

    pub fn set_onrepeat(&mut self, value: jsbind::Any) {
        self.inner.set("onrepeat", value);
    }
}
impl SVGAnimationElement {
    pub fn get_start_time(&self) -> f32 {
        self.inner.call("getStartTime", &[]).as_::<f32>()
    }
}
impl SVGAnimationElement {
    pub fn get_current_time(&self) -> f32 {
        self.inner.call("getCurrentTime", &[]).as_::<f32>()
    }
}
impl SVGAnimationElement {
    pub fn get_simple_duration(&self) -> f32 {
        self.inner.call("getSimpleDuration", &[]).as_::<f32>()
    }
}
impl SVGAnimationElement {
    pub fn begin_element(&self) -> jsbind::Undefined {
        self.inner
            .call("beginElement", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGAnimationElement {
    pub fn begin_element_at(&self, offset: f32) -> jsbind::Undefined {
        self.inner
            .call("beginElementAt", &[offset.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGAnimationElement {
    pub fn end_element(&self) -> jsbind::Undefined {
        self.inner
            .call("endElement", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGAnimationElement {
    pub fn end_element_at(&self, offset: f32) -> jsbind::Undefined {
        self.inner
            .call("endElementAt", &[offset.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGAnimationElement {
    pub fn required_extensions(&self) -> SVGStringList {
        self.inner.get("requiredExtensions").as_::<SVGStringList>()
    }
}
impl SVGAnimationElement {
    pub fn system_language(&self) -> SVGStringList {
        self.inner.get("systemLanguage").as_::<SVGStringList>()
    }
}
