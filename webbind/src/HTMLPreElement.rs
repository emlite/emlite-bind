use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLPreElement {
    inner: HTMLElement,
}
impl FromVal for HTMLPreElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLPreElement {
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
impl core::ops::Deref for HTMLPreElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLPreElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLPreElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLPreElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLPreElement> for emlite::Val {
    fn from(s: HTMLPreElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLPreElement);

impl HTMLPreElement {
    pub fn new() -> HTMLPreElement {
        Self {
            inner: emlite::Val::global("HTMLPreElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLPreElement {
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }

    pub fn set_width(&mut self, value: i32) {
        self.inner.set("width", value);
    }
}
