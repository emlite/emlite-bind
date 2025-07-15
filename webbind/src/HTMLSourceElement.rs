use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for HTMLSourceElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLSourceElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&HTMLSourceElement> for emlite::Val {
    fn from(s: &HTMLSourceElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLSourceElement);

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
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLSourceElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLSourceElement {
    pub fn srcset(&self) -> USVString {
        self.inner.get("srcset").as_::<USVString>()
    }

    pub fn set_srcset(&mut self, value: USVString) {
        self.inner.set("srcset", value);
    }
}
impl HTMLSourceElement {
    pub fn sizes(&self) -> DOMString {
        self.inner.get("sizes").as_::<DOMString>()
    }

    pub fn set_sizes(&mut self, value: DOMString) {
        self.inner.set("sizes", value);
    }
}
impl HTMLSourceElement {
    pub fn media(&self) -> DOMString {
        self.inner.get("media").as_::<DOMString>()
    }

    pub fn set_media(&mut self, value: DOMString) {
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
