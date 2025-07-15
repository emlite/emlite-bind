use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for HTMLBodyElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLBodyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLBodyElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLBodyElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLBodyElement> for emlite::Val {
    fn from(s: HTMLBodyElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLBodyElement> for emlite::Val {
    fn from(s: &HTMLBodyElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLBodyElement);

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
    pub fn onorientationchange(&self) -> Any {
        self.inner.get("onorientationchange").as_::<Any>()
    }

    pub fn set_onorientationchange(&mut self, value: &Any) {
        self.inner.set("onorientationchange", value);
    }
}
impl HTMLBodyElement {
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl HTMLBodyElement {
    pub fn link(&self) -> String {
        self.inner.get("link").as_::<String>()
    }

    pub fn set_link(&mut self, value: &str) {
        self.inner.set("link", value);
    }
}
impl HTMLBodyElement {
    pub fn v_link(&self) -> String {
        self.inner.get("vLink").as_::<String>()
    }

    pub fn set_v_link(&mut self, value: &str) {
        self.inner.set("vLink", value);
    }
}
impl HTMLBodyElement {
    pub fn a_link(&self) -> String {
        self.inner.get("aLink").as_::<String>()
    }

    pub fn set_a_link(&mut self, value: &str) {
        self.inner.set("aLink", value);
    }
}
impl HTMLBodyElement {
    pub fn bg_color(&self) -> String {
        self.inner.get("bgColor").as_::<String>()
    }

    pub fn set_bg_color(&mut self, value: &str) {
        self.inner.set("bgColor", value);
    }
}
impl HTMLBodyElement {
    pub fn background(&self) -> String {
        self.inner.get("background").as_::<String>()
    }

    pub fn set_background(&mut self, value: &str) {
        self.inner.set("background", value);
    }
}
impl HTMLBodyElement {
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    pub fn set_onportalactivate(&mut self, value: &Any) {
        self.inner.set("onportalactivate", value);
    }
}
