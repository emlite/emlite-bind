use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLObjectElement {
    inner: HTMLElement,
}
impl FromVal for HTMLObjectElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLObjectElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLObjectElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLObjectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLObjectElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLObjectElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLObjectElement> for emlite::Val {
    fn from(s: HTMLObjectElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLObjectElement);



impl HTMLObjectElement {
    pub fn new() -> HTMLObjectElement {
        Self {
            inner: emlite::Val::global("HTMLObjectElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLObjectElement {
    pub fn data(&self) -> USVString {
        self.inner.get("data").as_::<USVString>()
    }

    pub fn set_data(&mut self, value: USVString) {
        self.inner.set("data", value);
    }

}
impl HTMLObjectElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }

}
impl HTMLObjectElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }

}
impl HTMLObjectElement {
    pub fn form(&self) -> HTMLFormElement {
        self.inner.get("form").as_::<HTMLFormElement>()
    }

}
impl HTMLObjectElement {
    pub fn width(&self) -> DOMString {
        self.inner.get("width").as_::<DOMString>()
    }

    pub fn set_width(&mut self, value: DOMString) {
        self.inner.set("width", value);
    }

}
impl HTMLObjectElement {
    pub fn height(&self) -> DOMString {
        self.inner.get("height").as_::<DOMString>()
    }

    pub fn set_height(&mut self, value: DOMString) {
        self.inner.set("height", value);
    }

}
impl HTMLObjectElement {
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }

}
impl HTMLObjectElement {
    pub fn content_window(&self) -> Any {
        self.inner.get("contentWindow").as_::<Any>()
    }

}
impl HTMLObjectElement {
    pub fn get_svg_document(&self, ) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }

}
impl HTMLObjectElement {
    pub fn will_validate(&self) -> bool {
        self.inner.get("willValidate").as_::<bool>()
    }

}
impl HTMLObjectElement {
    pub fn validity(&self) -> ValidityState {
        self.inner.get("validity").as_::<ValidityState>()
    }

}
impl HTMLObjectElement {
    pub fn validation_message(&self) -> DOMString {
        self.inner.get("validationMessage").as_::<DOMString>()
    }

}
impl HTMLObjectElement {
    pub fn check_validity(&self, ) -> bool {
        self.inner.call("checkValidity", &[]).as_::<bool>()
    }

}
impl HTMLObjectElement {
    pub fn report_validity(&self, ) -> bool {
        self.inner.call("reportValidity", &[]).as_::<bool>()
    }

}
impl HTMLObjectElement {
    pub fn set_custom_validity(&self, error: DOMString) -> Undefined {
        self.inner.call("setCustomValidity", &[error.into(), ]).as_::<Undefined>()
    }

}
impl HTMLObjectElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }

}
impl HTMLObjectElement {
    pub fn archive(&self) -> DOMString {
        self.inner.get("archive").as_::<DOMString>()
    }

    pub fn set_archive(&mut self, value: DOMString) {
        self.inner.set("archive", value);
    }

}
impl HTMLObjectElement {
    pub fn code(&self) -> DOMString {
        self.inner.get("code").as_::<DOMString>()
    }

    pub fn set_code(&mut self, value: DOMString) {
        self.inner.set("code", value);
    }

}
impl HTMLObjectElement {
    pub fn declare(&self) -> bool {
        self.inner.get("declare").as_::<bool>()
    }

    pub fn set_declare(&mut self, value: bool) {
        self.inner.set("declare", value);
    }

}
impl HTMLObjectElement {
    pub fn hspace(&self) -> u32 {
        self.inner.get("hspace").as_::<u32>()
    }

    pub fn set_hspace(&mut self, value: u32) {
        self.inner.set("hspace", value);
    }

}
impl HTMLObjectElement {
    pub fn standby(&self) -> DOMString {
        self.inner.get("standby").as_::<DOMString>()
    }

    pub fn set_standby(&mut self, value: DOMString) {
        self.inner.set("standby", value);
    }

}
impl HTMLObjectElement {
    pub fn vspace(&self) -> u32 {
        self.inner.get("vspace").as_::<u32>()
    }

    pub fn set_vspace(&mut self, value: u32) {
        self.inner.set("vspace", value);
    }

}
impl HTMLObjectElement {
    pub fn code_base(&self) -> DOMString {
        self.inner.get("codeBase").as_::<DOMString>()
    }

    pub fn set_code_base(&mut self, value: DOMString) {
        self.inner.set("codeBase", value);
    }

}
impl HTMLObjectElement {
    pub fn code_type(&self) -> DOMString {
        self.inner.get("codeType").as_::<DOMString>()
    }

    pub fn set_code_type(&mut self, value: DOMString) {
        self.inner.set("codeType", value);
    }

}
impl HTMLObjectElement {
    pub fn use_map(&self) -> DOMString {
        self.inner.get("useMap").as_::<DOMString>()
    }

    pub fn set_use_map(&mut self, value: DOMString) {
        self.inner.set("useMap", value);
    }

}
impl HTMLObjectElement {
    pub fn border(&self) -> DOMString {
        self.inner.get("border").as_::<DOMString>()
    }

    pub fn set_border(&mut self, value: DOMString) {
        self.inner.set("border", value);
    }

}
