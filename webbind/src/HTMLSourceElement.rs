use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLSourceElement {
    inner: HTMLElement,
}
impl FromVal for HTMLSourceElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLSourceElement {
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
impl core::ops::Deref for HTMLSourceElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLSourceElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLSourceElement> for emlite::Val {
    fn from(s: HTMLSourceElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLSourceElement {
    pub fn new() -> HTMLSourceElement {
        Self {
            inner: emlite::Val::global("HTMLSourceElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLSourceElement {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLSourceElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLSourceElement {
    pub fn srcset(&self) -> jsbind::USVString {
        self.inner.get("srcset").as_::<jsbind::USVString>()
    }

    pub fn set_srcset(&mut self, value: jsbind::USVString) {
        self.inner.set("srcset", value);
    }
}
impl HTMLSourceElement {
    pub fn sizes(&self) -> jsbind::DOMString {
        self.inner.get("sizes").as_::<jsbind::DOMString>()
    }

    pub fn set_sizes(&mut self, value: jsbind::DOMString) {
        self.inner.set("sizes", value);
    }
}
impl HTMLSourceElement {
    pub fn media(&self) -> jsbind::DOMString {
        self.inner.get("media").as_::<jsbind::DOMString>()
    }

    pub fn set_media(&mut self, value: jsbind::DOMString) {
        self.inner.set("media", value);
    }
}
impl HTMLSourceElement {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLSourceElement {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
