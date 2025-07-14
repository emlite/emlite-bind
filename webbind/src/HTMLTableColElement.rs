use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLTableColElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTableColElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTableColElement {
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
impl core::ops::Deref for HTMLTableColElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTableColElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLTableColElement> for emlite::Val {
    fn from(s: HTMLTableColElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLTableColElement {
    pub fn new() -> HTMLTableColElement {
        Self {
            inner: emlite::Val::global("HTMLTableColElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTableColElement {
    pub fn span(&self) -> u32 {
        self.inner.get("span").as_::<u32>()
    }

    pub fn set_span(&mut self, value: u32) {
        self.inner.set("span", value);
    }
}
impl HTMLTableColElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("align", value);
    }
}
impl HTMLTableColElement {
    pub fn ch(&self) -> jsbind::DOMString {
        self.inner.get("ch").as_::<jsbind::DOMString>()
    }

    pub fn set_ch(&mut self, value: jsbind::DOMString) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableColElement {
    pub fn ch_off(&self) -> jsbind::DOMString {
        self.inner.get("chOff").as_::<jsbind::DOMString>()
    }

    pub fn set_ch_off(&mut self, value: jsbind::DOMString) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableColElement {
    pub fn v_align(&self) -> jsbind::DOMString {
        self.inner.get("vAlign").as_::<jsbind::DOMString>()
    }

    pub fn set_v_align(&mut self, value: jsbind::DOMString) {
        self.inner.set("vAlign", value);
    }
}
impl HTMLTableColElement {
    pub fn width(&self) -> jsbind::DOMString {
        self.inner.get("width").as_::<jsbind::DOMString>()
    }

    pub fn set_width(&mut self, value: jsbind::DOMString) {
        self.inner.set("width", value);
    }
}
