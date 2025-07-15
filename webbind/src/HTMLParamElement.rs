use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLParamElement {
    inner: HTMLElement,
}
impl FromVal for HTMLParamElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLParamElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLParamElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLParamElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLParamElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLParamElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLParamElement> for emlite::Val {
    fn from(s: HTMLParamElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLParamElement);

impl HTMLParamElement {
    pub fn new() -> HTMLParamElement {
        Self {
            inner: emlite::Val::global("HTMLParamElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLParamElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLParamElement {
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    pub fn set_value(&mut self, value: DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLParamElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLParamElement {
    pub fn value_type(&self) -> DOMString {
        self.inner.get("valueType").as_::<DOMString>()
    }

    pub fn set_value_type(&mut self, value: DOMString) {
        self.inner.set("valueType", value);
    }
}
