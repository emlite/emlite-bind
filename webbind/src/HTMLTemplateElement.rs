use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTemplateElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTemplateElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTemplateElement {
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
impl core::ops::Deref for HTMLTemplateElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTemplateElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLTemplateElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTemplateElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLTemplateElement> for emlite::Val {
    fn from(s: HTMLTemplateElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTemplateElement);

impl HTMLTemplateElement {
    pub fn new() -> HTMLTemplateElement {
        Self {
            inner: emlite::Val::global("HTMLTemplateElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTemplateElement {
    pub fn content(&self) -> DocumentFragment {
        self.inner.get("content").as_::<DocumentFragment>()
    }
}
impl HTMLTemplateElement {
    pub fn shadow_root_mode(&self) -> DOMString {
        self.inner.get("shadowRootMode").as_::<DOMString>()
    }

    pub fn set_shadow_root_mode(&mut self, value: DOMString) {
        self.inner.set("shadowRootMode", value);
    }
}
impl HTMLTemplateElement {
    pub fn shadow_root_delegates_focus(&self) -> bool {
        self.inner.get("shadowRootDelegatesFocus").as_::<bool>()
    }

    pub fn set_shadow_root_delegates_focus(&mut self, value: bool) {
        self.inner.set("shadowRootDelegatesFocus", value);
    }
}
impl HTMLTemplateElement {
    pub fn shadow_root_clonable(&self) -> bool {
        self.inner.get("shadowRootClonable").as_::<bool>()
    }

    pub fn set_shadow_root_clonable(&mut self, value: bool) {
        self.inner.set("shadowRootClonable", value);
    }
}
impl HTMLTemplateElement {
    pub fn shadow_root_serializable(&self) -> bool {
        self.inner.get("shadowRootSerializable").as_::<bool>()
    }

    pub fn set_shadow_root_serializable(&mut self, value: bool) {
        self.inner.set("shadowRootSerializable", value);
    }
}
impl HTMLTemplateElement {
    pub fn shadow_root_custom_element_registry(&self) -> DOMString {
        self.inner
            .get("shadowRootCustomElementRegistry")
            .as_::<DOMString>()
    }

    pub fn set_shadow_root_custom_element_registry(&mut self, value: DOMString) {
        self.inner.set("shadowRootCustomElementRegistry", value);
    }
}
