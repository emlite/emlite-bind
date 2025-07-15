use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFormElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFormElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFormElement {
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
impl core::ops::Deref for HTMLFormElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLFormElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLFormElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLFormElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLFormElement> for emlite::Val {
    fn from(s: HTMLFormElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLFormElement> for emlite::Val {
    fn from(s: &HTMLFormElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLFormElement);

impl HTMLFormElement {
    pub fn new() -> HTMLFormElement {
        Self {
            inner: emlite::Val::global("HTMLFormElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFormElement {
    pub fn accept_charset(&self) -> String {
        self.inner.get("acceptCharset").as_::<String>()
    }

    pub fn set_accept_charset(&mut self, value: &str) {
        self.inner.set("acceptCharset", value);
    }
}
impl HTMLFormElement {
    pub fn action(&self) -> String {
        self.inner.get("action").as_::<String>()
    }

    pub fn set_action(&mut self, value: &str) {
        self.inner.set("action", value);
    }
}
impl HTMLFormElement {
    pub fn autocomplete(&self) -> String {
        self.inner.get("autocomplete").as_::<String>()
    }

    pub fn set_autocomplete(&mut self, value: &str) {
        self.inner.set("autocomplete", value);
    }
}
impl HTMLFormElement {
    pub fn enctype(&self) -> String {
        self.inner.get("enctype").as_::<String>()
    }

    pub fn set_enctype(&mut self, value: &str) {
        self.inner.set("enctype", value);
    }
}
impl HTMLFormElement {
    pub fn encoding(&self) -> String {
        self.inner.get("encoding").as_::<String>()
    }

    pub fn set_encoding(&mut self, value: &str) {
        self.inner.set("encoding", value);
    }
}
impl HTMLFormElement {
    pub fn method(&self) -> String {
        self.inner.get("method").as_::<String>()
    }

    pub fn set_method(&mut self, value: &str) {
        self.inner.set("method", value);
    }
}
impl HTMLFormElement {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLFormElement {
    pub fn no_validate(&self) -> bool {
        self.inner.get("noValidate").as_::<bool>()
    }

    pub fn set_no_validate(&mut self, value: bool) {
        self.inner.set("noValidate", value);
    }
}
impl HTMLFormElement {
    pub fn target(&self) -> String {
        self.inner.get("target").as_::<String>()
    }

    pub fn set_target(&mut self, value: &str) {
        self.inner.set("target", value);
    }
}
impl HTMLFormElement {
    pub fn rel(&self) -> String {
        self.inner.get("rel").as_::<String>()
    }

    pub fn set_rel(&mut self, value: &str) {
        self.inner.set("rel", value);
    }
}
impl HTMLFormElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLFormElement {
    pub fn elements(&self) -> HTMLFormControlsCollection {
        self.inner
            .get("elements")
            .as_::<HTMLFormControlsCollection>()
    }
}
impl HTMLFormElement {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl HTMLFormElement {
    pub fn submit(&self) -> Undefined {
        self.inner.call("submit", &[]).as_::<Undefined>()
    }
}
impl HTMLFormElement {
    pub fn request_submit0(&self) -> Undefined {
        self.inner.call("requestSubmit", &[]).as_::<Undefined>()
    }

    pub fn request_submit1(&self, submitter: &HTMLElement) -> Undefined {
        self.inner
            .call("requestSubmit", &[submitter.into()])
            .as_::<Undefined>()
    }
}
impl HTMLFormElement {
    pub fn reset(&self) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }
}
impl HTMLFormElement {
    pub fn check_validity(&self) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }
}
impl HTMLFormElement {
    pub fn report_validity(&self) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }
}
