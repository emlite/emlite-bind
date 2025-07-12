use super::*;

#[derive(Clone, Debug)]
pub struct HTMLHRElement {
    inner: HTMLElement,
}
impl FromVal for HTMLHRElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLHRElement {
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
impl std::ops::Deref for HTMLHRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLHRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLHRElement> for emlite::Val {
    fn from(s: HTMLHRElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLHRElement {
    pub fn new() -> HTMLHRElement {
        Self {
            inner: emlite::Val::global("HTMLHRElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLHRElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLHRElement {
    pub fn color(&self) -> jsbind::DOMString {
        self.inner.get("color").as_::<jsbind::DOMString>()
    }

    pub fn set_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("color", value);
    }
}
impl HTMLHRElement {
    pub fn no_shade(&self) -> bool {
        self.inner.get("noShade").as_::<bool>()
    }

    pub fn set_no_shade(&mut self, value: bool) {
        self.inner.set("noShade", value);
    }
}
impl HTMLHRElement {
    pub fn size(&self) -> jsbind::DOMString {
        self.inner.get("size").as_::<jsbind::DOMString>()
    }

    pub fn set_size(&mut self, value: jsbind::DOMString) {
        self.inner.set("size", value);
    }
}
impl HTMLHRElement {
    pub fn width(&self) -> jsbind::DOMString {
        self.inner.get("width").as_::<jsbind::DOMString>()
    }

    pub fn set_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("width", value);
    }
}
