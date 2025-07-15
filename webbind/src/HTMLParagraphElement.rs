use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLParagraphElement {
    inner: HTMLElement,
}
impl FromVal for HTMLParagraphElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLParagraphElement {
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
impl core::ops::Deref for HTMLParagraphElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLParagraphElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLParagraphElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLParagraphElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLParagraphElement> for emlite::Val {
    fn from(s: HTMLParagraphElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLParagraphElement> for emlite::Val {
    fn from(s: &HTMLParagraphElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLParagraphElement);

impl HTMLParagraphElement {
    pub fn new() -> HTMLParagraphElement {
        Self {
            inner: emlite::Val::global("HTMLParagraphElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLParagraphElement {
    pub fn align(&self) -> DOMString {
        self.inner.get("align").as_::<DOMString>()
    }

    pub fn set_align(&mut self, value: DOMString) {
        self.inner.set("align", value);
    }
}
