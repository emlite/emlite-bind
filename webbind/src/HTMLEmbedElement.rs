use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<HTMLEmbedElement> for emlite::Val {
    fn from(s: HTMLEmbedElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLEmbedElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLEmbedElement {
    pub fn width(&self) -> jsbind::DOMString {
        self.inner.get("width").as_::<jsbind::DOMString>()
    }

    pub fn set_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("width", value);
    }
}
impl HTMLEmbedElement {
    pub fn height(&self) -> jsbind::DOMString {
        self.inner.get("height").as_::<jsbind::DOMString>()
    }

    pub fn set_height(&mut self, value: jsbind::DOMString) {
        self.inner.set("height", value);
    }
}
impl HTMLEmbedElement {
    pub fn get_svg_document(&self) -> Document {
        self.inner.call("getSVGDocument", &[]).as_::<Document>()
    }
}
impl HTMLEmbedElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLEmbedElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
