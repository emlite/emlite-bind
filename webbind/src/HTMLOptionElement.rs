use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOptionElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOptionElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLOptionElement {
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
impl core::ops::Deref for HTMLOptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOptionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLOptionElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLOptionElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLOptionElement> for emlite::Val {
    fn from(s: HTMLOptionElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLOptionElement> for emlite::Val {
    fn from(s: &HTMLOptionElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOptionElement);

impl HTMLOptionElement {
    pub fn new() -> HTMLOptionElement {
        Self {
            inner: emlite::Val::global("HTMLOptionElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOptionElement {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
impl HTMLOptionElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLOptionElement {
    pub fn label(&self) -> DOMString {
        self.inner.get("label").as_::<DOMString>()
    }

    pub fn set_label(&mut self, value: DOMString) {
        self.inner.set("label", value);
    }
}
impl HTMLOptionElement {
    pub fn default_selected(&self) -> bool {
        self.inner.get("defaultSelected").as_::<bool>()
    }

    pub fn set_default_selected(&mut self, value: bool) {
        self.inner.set("defaultSelected", value);
    }
}
impl HTMLOptionElement {
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
impl HTMLOptionElement {
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    pub fn set_value(&mut self, value: DOMString) {
        self.inner.set("value", value);
    }
}
impl HTMLOptionElement {
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    pub fn set_text(&mut self, value: DOMString) {
        self.inner.set("text", value);
    }
}
impl HTMLOptionElement {
    pub fn index(&self) -> i32 {
        self.inner.get("index").as_::<i32>()
    }
}
