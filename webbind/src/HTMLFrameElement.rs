use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for HTMLFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLFrameElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLFrameElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLFrameElement> for emlite::Val {
    fn from(s: HTMLFrameElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLFrameElement> for emlite::Val {
    fn from(s: &HTMLFrameElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLFrameElement);

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
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLFrameElement {
    pub fn scrolling(&self) -> DOMString {
        self.inner.get("scrolling").as_::<DOMString>()
    }

    pub fn set_scrolling(&mut self, value: DOMString) {
        self.inner.set("scrolling", value);
    }
}
impl HTMLFrameElement {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLFrameElement {
    pub fn frame_border(&self) -> DOMString {
        self.inner.get("frameBorder").as_::<DOMString>()
    }

    pub fn set_frame_border(&mut self, value: DOMString) {
        self.inner.set("frameBorder", value);
    }
}
impl HTMLFrameElement {
    pub fn long_desc(&self) -> USVString {
        self.inner.get("longDesc").as_::<USVString>()
    }

    pub fn set_long_desc(&mut self, value: USVString) {
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
    pub fn content_window(&self) -> Any {
        self.inner.get("contentWindow").as_::<Any>()
    }
}
impl HTMLFrameElement {
    pub fn margin_height(&self) -> DOMString {
        self.inner.get("marginHeight").as_::<DOMString>()
    }

    pub fn set_margin_height(&mut self, value: DOMString) {
        self.inner.set("marginHeight", value);
    }
}
impl HTMLFrameElement {
    pub fn margin_width(&self) -> DOMString {
        self.inner.get("marginWidth").as_::<DOMString>()
    }

    pub fn set_margin_width(&mut self, value: DOMString) {
        self.inner.set("marginWidth", value);
    }
}
