use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOutputElement {
    inner: HTMLElement,
}
impl FromVal for HTMLOutputElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLOutputElement {
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
impl core::ops::Deref for HTMLOutputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOutputElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLOutputElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLOutputElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLOutputElement> for emlite::Val {
    fn from(s: HTMLOutputElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLOutputElement> for emlite::Val {
    fn from(s: &HTMLOutputElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOutputElement);

impl HTMLOutputElement {
    pub fn new() -> HTMLOutputElement {
        Self {
            inner: emlite::Val::global("HTMLOutputElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLOutputElement {
    pub fn html_for(&self) -> DOMTokenList {
        self.inner.get("htmlFor").as_::<DOMTokenList>()
    }
}
impl HTMLOutputElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }
}
impl HTMLOutputElement {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLOutputElement {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }
}
impl HTMLOutputElement {
    pub fn default_value(&self) -> String {
        self.inner.get("defaultValue").as_::<String>()
    }

    pub fn set_default_value(&mut self, value: &str) {
        self.inner.set("defaultValue", value);
    }
}
impl HTMLOutputElement {
    pub fn value(&self) -> String {
        self.inner.get("value").as_::<String>()
    }

    pub fn set_value(&mut self, value: &str) {
        self.inner.set("value", value);
    }
}
impl HTMLOutputElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }
}
impl HTMLOutputElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }
}
impl HTMLOutputElement {
    pub fn validation_message(&self) -> String {
        self.inner.get("validationMessage").as_::<String>()
    }
}
impl HTMLOutputElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLOutputElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
impl HTMLOutputElement {
    pub fn set_custom_validity(&self, error: &str) -> Undefined {
        self.inner
            .call("setCustomValidity", &[error.into()])
            .as_::<Undefined>()
    }
}
impl HTMLOutputElement {
    pub fn labels(&self) -> NodeList {
        self.inner.get("labels").as_::<NodeList>()
    }
}
