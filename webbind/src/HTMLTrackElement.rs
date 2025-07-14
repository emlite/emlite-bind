use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTMLTrackElement {
    inner: HTMLElement,
}
impl FromVal for HTMLTrackElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLTrackElement {
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
impl core::ops::Deref for HTMLTrackElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLTrackElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLTrackElement> for emlite::Val {
    fn from(s: HTMLTrackElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLTrackElement {
    pub fn new() -> HTMLTrackElement {
        Self {
            inner: emlite::Val::global("HTMLTrackElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLTrackElement {
    pub fn kind(&self) -> jsbind::DOMString {
        self.inner.get("kind").as_::<jsbind::DOMString>()
    }

    pub fn set_kind(&mut self, value: jsbind::DOMString) {
        self.inner.set("kind", value);
    }
}
impl HTMLTrackElement {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLTrackElement {
    pub fn srclang(&self) -> jsbind::DOMString {
        self.inner.get("srclang").as_::<jsbind::DOMString>()
    }

    pub fn set_srclang(&mut self, value: jsbind::DOMString) {
        self.inner.set("srclang", value);
    }
}
impl HTMLTrackElement {
    pub fn label(&self) -> jsbind::DOMString {
        self.inner.get("label").as_::<jsbind::DOMString>()
    }

    pub fn set_label(&mut self, value: jsbind::DOMString) {
        self.inner.set("label", value);
    }
}
impl HTMLTrackElement {
    pub fn default(&self) -> bool {
        self.inner.get("default").as_::<bool>()
    }

    pub fn set_default(&mut self, value: bool) {
        self.inner.set("default", value);
    }
}
impl HTMLTrackElement {
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl HTMLTrackElement {
    pub fn track(&self) -> TextTrack {
        self.inner.get("track").as_::<TextTrack>()
    }
}
