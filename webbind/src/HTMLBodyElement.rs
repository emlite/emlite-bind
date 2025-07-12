use super::*;

#[derive(Clone, Debug)]
pub struct HTMLBodyElement {
    inner: HTMLElement,
}
impl FromVal for HTMLBodyElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLBodyElement {
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
impl std::ops::Deref for HTMLBodyElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLBodyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLBodyElement> for emlite::Val {
    fn from(s: HTMLBodyElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLBodyElement {
    pub fn new() -> HTMLBodyElement {
        Self {
            inner: emlite::Val::global("HTMLBodyElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLBodyElement {
    pub fn onorientationchange(&self) -> jsbind::Any {
        self.inner.get("onorientationchange").as_::<jsbind::Any>()
    }

    pub fn set_onorientationchange(&mut self, value: jsbind::Any) {
        self.inner.set("onorientationchange", value);
    }
}
impl HTMLBodyElement {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }

    pub fn set_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("text", value);
    }
}
impl HTMLBodyElement {
    pub fn link(&self) -> jsbind::DOMString {
        self.inner.get("link").as_::<jsbind::DOMString>()
    }

    pub fn set_link(&mut self, value: jsbind::DOMString) {
        self.inner.set("link", value);
    }
}
impl HTMLBodyElement {
    pub fn v_link(&self) -> jsbind::DOMString {
        self.inner.get("vLink").as_::<jsbind::DOMString>()
    }

    pub fn set_v_link(&mut self, value: jsbind::DOMString) {
        self.inner.set("vLink", value);
    }
}
impl HTMLBodyElement {
    pub fn a_link(&self) -> jsbind::DOMString {
        self.inner.get("aLink").as_::<jsbind::DOMString>()
    }

    pub fn set_a_link(&mut self, value: jsbind::DOMString) {
        self.inner.set("aLink", value);
    }
}
impl HTMLBodyElement {
    pub fn bg_color(&self) -> jsbind::DOMString {
        self.inner.get("bgColor").as_::<jsbind::DOMString>()
    }

    pub fn set_bg_color(&mut self, value: jsbind::DOMString) {
        self.inner.set("bgColor", value);
    }
}
impl HTMLBodyElement {
    pub fn background(&self) -> jsbind::DOMString {
        self.inner.get("background").as_::<jsbind::DOMString>()
    }

    pub fn set_background(&mut self, value: jsbind::DOMString) {
        self.inner.set("background", value);
    }
}
impl HTMLBodyElement {
    pub fn onportalactivate(&self) -> jsbind::Any {
        self.inner.get("onportalactivate").as_::<jsbind::Any>()
    }

    pub fn set_onportalactivate(&mut self, value: jsbind::Any) {
        self.inner.set("onportalactivate", value);
    }
}
