use super::*;

#[derive(Clone, Debug)]
pub struct HTMLFrameElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFrameElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFrameElement {
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
impl std::ops::Deref for HTMLFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLFrameElement> for emlite::Val {
    fn from(s: HTMLFrameElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLFrameElement {
    pub fn new() -> HTMLFrameElement {
        Self {
            inner: emlite::Val::global("HTMLFrameElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFrameElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLFrameElement {
    pub fn scrolling(&self) -> jsbind::DOMString {
        self.inner.get("scrolling").as_::<jsbind::DOMString>()
    }

    pub fn set_scrolling(&mut self, value: jsbind::DOMString) {
        self.inner.set("scrolling", value);
    }
}
impl HTMLFrameElement {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLFrameElement {
    pub fn frame_border(&self) -> jsbind::DOMString {
        self.inner.get("frameBorder").as_::<jsbind::DOMString>()
    }

    pub fn set_frame_border(&mut self, value: jsbind::DOMString) {
        self.inner.set("frameBorder", value);
    }
}
impl HTMLFrameElement {
    pub fn long_desc(&self) -> jsbind::USVString {
        self.inner.get("longDesc").as_::<jsbind::USVString>()
    }

    pub fn set_long_desc(&mut self, value: jsbind::USVString) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLFrameElement {
    pub fn no_resize(&self) -> bool {
        self.inner.get("noResize").as_::<bool>()
    }

    pub fn set_no_resize(&mut self, value: bool) {
        self.inner.set("noResize", value);
    }
}
impl HTMLFrameElement {
    pub fn content_document(&self) -> Document {
        self.inner.get("contentDocument").as_::<Document>()
    }
}
impl HTMLFrameElement {
    pub fn content_window(&self) -> jsbind::Any {
        self.inner.get("contentWindow").as_::<jsbind::Any>()
    }
}
impl HTMLFrameElement {
    pub fn margin_height(&self) -> jsbind::DOMString {
        self.inner.get("marginHeight").as_::<jsbind::DOMString>()
    }

    pub fn set_margin_height(&mut self, value: jsbind::DOMString) {
        self.inner.set("marginHeight", value);
    }
}
impl HTMLFrameElement {
    pub fn margin_width(&self) -> jsbind::DOMString {
        self.inner.get("marginWidth").as_::<jsbind::DOMString>()
    }

    pub fn set_margin_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("marginWidth", value);
    }
}
