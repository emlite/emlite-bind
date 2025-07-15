use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLBRElement {
    inner: HTMLElement,
}
impl FromVal for HTMLBRElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLBRElement {
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
impl core::ops::Deref for HTMLBRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLBRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLBRElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLBRElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLBRElement> for emlite::Val {
    fn from(s: HTMLBRElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLBRElement> for emlite::Val {
    fn from(s: &HTMLBRElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLBRElement);

impl HTMLBRElement {
    pub fn new() -> HTMLBRElement {
        Self {
            inner: emlite::Val::global("HTMLBRElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLBRElement {
    pub fn clear(&self) -> DOMString {
        self.inner.get("clear").as_::<DOMString>()
    }

    pub fn set_clear(&mut self, value: DOMString) {
        self.inner.set("clear", value);
    }
}
