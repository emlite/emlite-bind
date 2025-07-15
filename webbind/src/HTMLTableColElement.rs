use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for HTMLTableColElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLTableColElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&HTMLTableColElement> for emlite::Val {
    fn from(s: &HTMLTableColElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLTableColElement);

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
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
impl HTMLTableColElement {
    pub fn ch(&self) -> String {
        self.inner.get("ch").as_::<String>()
    }

    pub fn set_ch(&mut self, value: &str) {
        self.inner.set("ch", value);
    }
}
impl HTMLTableColElement {
    pub fn ch_off(&self) -> String {
        self.inner.get("chOff").as_::<String>()
    }

    pub fn set_ch_off(&mut self, value: &str) {
        self.inner.set("chOff", value);
    }
}
impl HTMLTableColElement {
    pub fn v_align(&self) -> String {
        self.inner.get("vAlign").as_::<String>()
    }

    pub fn set_v_align(&mut self, value: &str) {
        self.inner.set("vAlign", value);
    }
}
impl HTMLTableColElement {
    pub fn width(&self) -> String {
        self.inner.get("width").as_::<String>()
    }

    pub fn set_width(&mut self, value: &str) {
        self.inner.set("width", value);
    }
}
