use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLEmbedElement {
    inner: HTMLElement,
}
impl FromVal for HTMLEmbedElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLEmbedElement {
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
impl core::ops::Deref for HTMLEmbedElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLEmbedElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLEmbedElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLEmbedElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLEmbedElement> for emlite::Val {
    fn from(s: HTMLEmbedElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLEmbedElement);

impl HTMLEmbedElement {
    pub fn new() -> HTMLEmbedElement {
        Self {
            inner: emlite::Val::global("HTMLEmbedElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLEmbedElement {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLEmbedElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLEmbedElement {
    pub fn width(&self) -> DOMString {
        self.inner.get("width").as_::<DOMString>()
    }

    pub fn set_width(&mut self, value: DOMString) {
        self.inner.set("width", value);
    }
}
impl HTMLEmbedElement {
    pub fn height(&self) -> DOMString {
        self.inner.get("height").as_::<DOMString>()
    }

    pub fn set_height(&mut self, value: DOMString) {
        self.inner.set("height", value);
    }
}
impl HTMLEmbedElement {
    pub fn get_svg_document(&self) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }
}
impl HTMLEmbedElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLEmbedElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
